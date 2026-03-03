use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};
use futures::future::LocalBoxFuture;
use serde::Deserialize;
use jsonwebtoken::{decode, decode_header, Validation, Algorithm, DecodingKey};
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    roles: Option<Vec<String>>,
    aud: Option<Vec<String>>,
    iss: String,
    exp: usize,
}

pub enum AuthUser {
    Oidc { sub: String, roles: Vec<String> },
    Admin,
    Anonymous,
}

impl AuthUser {
    pub fn is_authenticated(&self) -> bool {
        matches!(self, AuthUser::Oidc { .. } | AuthUser::Admin)
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        Box::pin(async move {
            match auth_header {
                Some(header) if header.starts_with("ApiToken ") => {
                    let token = header.trim_start_matches("ApiToken ").trim();
                    match std::env::var("MAKO_ADMIN_TOKEN") {
                        Ok(admin_token) if token == admin_token => Ok(AuthUser::Admin),
                        _ => Err(ErrorUnauthorized("Invalid admin token")),
                    }
                }

                Some(header) if header.starts_with("Bearer ") => {
                    let token = header.trim_start_matches("Bearer ").trim();

                    let issuer = std::env::var("OIDC_ISSUER")
                        .map_err(|_| ErrorUnauthorized("Missing OIDC_ISSUER"))?;
                    let client_id = std::env::var("OIDC_CLIENT_ID")
                        .map_err(|_| ErrorUnauthorized("Missing OIDC_CLIENT_ID"))?;

                    // Fetch JWKS from issuer (remote)
                    let jwks_url = format!("{}/.well-known/jwks.json", issuer);
                    let jwks: serde_json::Value = Client::new()
                        .get(&jwks_url)
                        .send()
                        .await
                        .map_err(|e| ErrorUnauthorized(format!("Failed to fetch JWKS: {}", e)))?
                        .json()
                        .await
                        .map_err(|e| ErrorUnauthorized(format!("Failed to parse JWKS: {}", e)))?;

                    // Extract key id from token header
                    let header = decode_header(token).map_err(|_| ErrorUnauthorized("Invalid token header"))?;
                    let kid = header.kid.ok_or_else(|| ErrorUnauthorized("No kid in token header"))?;

                    // Find the correct key in JWKS
                    let key_data = jwks["keys"]
                        .as_array()
                        .and_then(|keys| keys.iter().find(|k| k["kid"] == kid))
                        .ok_or_else(|| ErrorUnauthorized("Key not found in JWKS"))?;

                    let n = key_data["n"].as_str().ok_or_else(|| ErrorUnauthorized("Invalid key data"))?;
                    let e = key_data["e"].as_str().ok_or_else(|| ErrorUnauthorized("Invalid key data"))?;

                    let decoding_key = DecodingKey::from_rsa_components(n, e).map_err(|_| ErrorUnauthorized("Failed to create decoding key"))?;
                    let mut validation = Validation::new(Algorithm::RS256);
                    validation.set_audience(&[client_id.as_str()]);
                    validation.set_issuer(&[issuer.as_str()]);

                    let token_data = decode::<Claims>(token, &decoding_key, &validation)
                        .map_err(|e| ErrorUnauthorized(format!("Token verification failed: {}", e)))?;

                    Ok(AuthUser::Oidc {
                        sub: token_data.claims.sub,
                        roles: token_data.claims.roles.unwrap_or_default(),
                    })
                }

                _ => Ok(AuthUser::Anonymous),
            }
        })
    }
}