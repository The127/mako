use crate::repositories::namespaces::Namespace;
use crate::repositories::rqlite::new_context;
use actix_web::{get, post, web, HttpResponse, Responder};
use shared::dtos::namespaces::CreateNamespaceDto;
use crate::extractors::auth::AuthUser;

#[post("/namespaces")]
async fn create_namespace(
    request_dto: web::Json<CreateNamespaceDto>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
) -> Result<HttpResponse, actix_web::error::Error> {
    match user {
        AuthUser::Anonymous => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        AuthUser::Oidc { .. } => return Err(actix_web::error::ErrorUnauthorized("Unauthorized: TODO")),
        AuthUser::Admin => (),
    }

    let mut ctx = new_context(con.into_inner());

    ctx.namespaces()
        .insert(Namespace::new(request_dto.path.clone()));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/namespaces")]
async fn list_namespaces() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
