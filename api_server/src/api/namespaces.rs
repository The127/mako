use crate::auth::AuthUser;
use crate::repositories::namespaces::Namespace;
use crate::repositories::rqlite::new_context;
use actix_web::{delete, get, put, web, HttpResponse};
use shared::dtos::namespaces::{NamespaceDto, NamespaceListDto, NamespacePath};
use shared::dtos::values::{ValueDto, ValueListDto};
use crate::auth;
use crate::auth::OperationType;

#[put("/v1/namespaces/{path:.+}")]
async fn create_namespace(
    ns: web::Path<NamespacePath>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let mut ctx = new_context(con.into_inner());

    ctx.namespaces()
        .insert_if_not_exists(Namespace::new(ns.into_inner().path));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/v1/namespaces")]
async fn list_namespaces(
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let ctx = new_context(con.into_inner());

    let namespaces = ctx.namespaces().list()?;

    Ok(HttpResponse::Ok().json(NamespaceListDto {
        namespaces: namespaces
            .into_iter()
            .map(|ns| NamespaceDto{
                path: ns.path(),
            })
            .collect(),
    }))
}

#[get("/v1/namespaces/{path:.+}")]
async fn list_namespace_kvs(
    ns: web::Path<NamespacePath>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let ctx = new_context(con.into_inner());

    let values = ctx.values().list(ns.into_inner().path.as_ref())?;

    Ok(HttpResponse::Ok().json(ValueListDto {
        values: values
            .into_iter()
            .map(|v| ValueDto {
                key: v.key(),
                value: v.value(),
                version: v.version() as u64,
            })
            .collect(),
    }))
}

#[delete("/v1/namespaces/{path:.+}")]
async fn delete_namespace(
    ns: web::Path<NamespacePath>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,  
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let mut ctx = new_context(con.into_inner());

    ctx.namespaces().delete_if_exists(ns.into_inner().path.as_ref());

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}
