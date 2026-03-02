use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use futures::future::LocalBoxFuture;
use jwt_verify::{JwtVerifier, OidcJwtVerifier, OidcProviderConfig};

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
                    let token = header.trim_start_matches("Bearer ").trim().to_string();

                    // Build OIDC verifier
                    let issuer = std::env::var("OIDC_ISSUER").unwrap();
                    let client_ids = vec![std::env::var("OIDC_CLIENT_ID").unwrap()];

                    let provider = OidcProviderConfig::new(&issuer, None, &client_ids, None)
                        .map_err(|_| ErrorUnauthorized("Invalid OIDC config"))?;
                    let verifier = OidcJwtVerifier::new(vec![provider])
                        .map_err(|_| ErrorUnauthorized("Failed to create verifier"))?;

                    // Verify token
                    let claims = verifier.verify_id_token(&token).await
                        .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

                    let sub = claims.get_sub().to_string();

                    // Extract roles claim (assume it’s an array of strings)
                    let roles: Vec<String> = claims.get_custom_claim("roles")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|val| val.as_str().map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();

                    Ok(AuthUser::Oidc { sub, roles })
                }

                _ => Ok(AuthUser::Anonymous),
            }
        })
    }
}