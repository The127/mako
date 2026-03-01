use crate::repositories::rqlite::new_context;
use crate::repositories::values::Value;
use actix_web::{post, web, HttpResponse};
use shared::dtos::values::CreateValueDto;
use crate::extractors::auth::AuthUser;

#[post("/values")]
async fn create_value(
    request_dto: web::Json<CreateValueDto>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
) -> Result<HttpResponse, actix_web::error::Error> {
    match user {
        AuthUser::Anonymous => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        AuthUser::Oidc { .. } => return Err(actix_web::error::ErrorUnauthorized("Unauthorized: TODO")),
        AuthUser::Admin => (),
    }

    let mut ctx = new_context(con.into_inner());

    ctx.values()
        .insert(Value::new(request_dto.path.clone(), request_dto.key.clone(), request_dto.value.clone()));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}
