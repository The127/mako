use crate::extractors::auth::AuthUser;
use crate::repositories::rqlite::new_context;
use crate::repositories::values::Value;
use actix_web::{HttpResponse, put, web};
use shared::dtos::values::{CreateValueDto, NamespacedKey};

#[put("/v1/kv/{path:.+}/{key}")]
async fn set_value(
    ns_key: web::Path<NamespacedKey>,
    request_dto: web::Json<CreateValueDto>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
) -> Result<HttpResponse, actix_web::error::Error> {
    match user {
        AuthUser::Anonymous => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        AuthUser::Oidc { .. } => {
            return Err(actix_web::error::ErrorUnauthorized("Unauthorized: TODO"));
        }
        AuthUser::Admin => (),
    }

    let mut ctx = new_context(con.into_inner());

    ctx.values().insert(Value::new(
        ns_key.path.clone(),
        ns_key.key.clone(),
        request_dto.value.clone(),
    ));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}
