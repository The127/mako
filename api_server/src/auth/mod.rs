use actix_web::{Error, FromRequest, HttpRequest};
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;
use crate::repositories::DbContext;
use crate::repositories::permissions::PermissionType;

pub struct OidcConfiguration {
    pub admin_role: String,
    pub writer_role: String,
    pub reader_role: String,
    pub issuer: String,
    pub client_id: String,
}

pub enum OperationType {
    Read,
    Write,
    Admin,
}

pub fn has_access(
    user: &AuthUser,
    oidc_config: &OidcConfiguration,
    operation_type: OperationType,
) -> bool
{
    match user {
        AuthUser::Admin => true,
        AuthUser::Oidc { sub, roles } => {
            if roles.contains(&oidc_config.admin_role) {
                true
            } else {
                match operation_type {
                    OperationType::Read => roles.contains(&oidc_config.reader_role),
                    OperationType::Write => roles.contains(&oidc_config.writer_role),
                    OperationType::Admin => false,
                }
            }
        }
        AuthUser::Anonymous => false,
    }
}

pub fn has_access_to_value(
    ctx: &Box<dyn DbContext>,
    path: &str,
    user: &AuthUser,
    oidc_config: &OidcConfiguration,
    operation_type: OperationType,
) -> Result<bool, Box<dyn std::error::Error>> {
    if has_access(user, oidc_config, OperationType::Read) {
        match user {
            AuthUser::Admin => Ok(true),
            AuthUser::Oidc { sub, roles } => {
                let permission = ctx.permissions().get(sub, path)?;
                match permission {
                    Some(permission) => {
                        match operation_type {
                            OperationType::Read => Ok(permission.has_permission(PermissionType::Read)),
                            OperationType::Write => Ok(permission.has_permission(PermissionType::Write)),
                            OperationType::Admin => Ok(false),
                        }
                    },
                    None => Ok(false),
                }
            },
            AuthUser::Anonymous => Ok( false),
        }
    } else {
        Ok(false)
    }
}


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
        let oidc_config = req.app_data::<OidcConfiguration>().unwrap();

        let issuer = oidc_config.issuer.clone();
        let client_id = oidc_config.client_id.clone();

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
