use crate::repositories::namespaces::Namespace;
use crate::repositories::rqlite::new_context;
use actix_web::{get, put, web, HttpResponse, Responder};
use shared::dtos::namespaces::NamespacePath;
use crate::extractors::auth::AuthUser;

#[put("/v1/namespaces/{path:.+}")]
async fn create_namespace(
    path: web::Path<NamespacePath>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
) -> Result<HttpResponse, actix_web::error::Error> {
    match user {
        AuthUser::Anonymous => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        AuthUser::Oidc { .. } => return Err(actix_web::error::ErrorUnauthorized("Unauthorized: TODO")),
        AuthUser::Admin => (),
    }

    let path = path.into_inner().path;

    let mut ctx = new_context(con.into_inner());

    ctx.namespaces()
        .insert_if_not_exists(Namespace::new(path));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/v1/namespaces")]
async fn list_namespaces() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
