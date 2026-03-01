use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use futures::future::{Ready, ready};

pub enum AuthUser {
    Oidc { sub: String, scopes: Vec<String> },
    Admin,
    Anonymous,
}

impl AuthUser {
    pub fn is_authenticated(&self) -> bool {
        match self {
            AuthUser::Oidc { .. } => true,
            AuthUser::Admin => true,
            AuthUser::Anonymous => false,
        }
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        match auth_header {
            Some(header) => {
                if header.starts_with("ApiToken ") {
                    let admin_token = std::env::var("MAKO_ADMIN_TOKEN");
                    match admin_token {
                        Ok(token) => {
                            if header.split_at(9).1 != token {
                                ready(Err(ErrorUnauthorized("Invalid authorization header")))
                            } else {
                                ready(Ok(AuthUser::Admin))
                            }
                        }
                        Err(_) => ready(Err(ErrorUnauthorized("Invalid authorization header"))),
                    }
                } else {
                    ready(Err(ErrorUnauthorized("Invalid authorization header")))
                }
            }
            None => ready(Ok(AuthUser::Anonymous)),
        }
    }
}
