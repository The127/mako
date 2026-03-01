use crate::extractors::auth::AuthUser;
use crate::repositories::namespaces::Namespace;
use crate::repositories::rqlite::new_context;
use actix_web::{get, put, web, HttpResponse};
use shared::dtos::namespaces::NamespacePath;
use shared::dtos::values::{ValueDto, ValueListDto};

#[put("/v1/namespaces/{path:.+}")]
async fn create_namespace(
    ns: web::Path<NamespacePath>,
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

    ctx.namespaces()
        .insert_if_not_exists(Namespace::new(ns.into_inner().path));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/v1/namespaces/{path:.+}")]
async fn list_namespaces(
    ns: web::Path<NamespacePath>,
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

    let ctx = new_context(con.into_inner());

    let values = ctx.values().list(ns.into_inner().path.as_ref())?;

    Ok(HttpResponse::Ok().json(ValueListDto {
        values: values
            .into_iter()
            .map(|v| ValueDto {
                key: v.key(),
                value: v.value(),
            })
            .collect(),
    }))
}
