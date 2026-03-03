use crate::extractors::auth::AuthUser;
use crate::repositories::permissions::{Permission, PermissionType};
use crate::repositories::rqlite::new_context;
use actix_web::{HttpResponse, delete, get, put, web};
use shared::dtos::namespaces::NamespacePath;
use shared::dtos::permissions::{CreatePermissionDto, NamespacedSubject, PermissionDto, PermissionListDto};
use std::str::FromStr;
use crate::auth;
use crate::auth::OperationType;

impl From<Permission> for PermissionDto {
    fn from(value: Permission) -> Self {
        PermissionDto {
            path: value.path().to_string(),
            subject_id: value.subject_id().to_string(),
            permissions: value
                .permissions()
                .iter()
                .map(|p| shared::dtos::permissions::PermissionType::from_str(p.to_string().as_str()).unwrap())
                .collect(),
        }
    }
}

#[put("/v1/acl/{path:.+}/{subject_id}")]
async fn set_acl(
    ns_subject: web::Path<NamespacedSubject>,
    request_dto: web::Json<CreatePermissionDto>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let mut ctx = new_context(con.into_inner());

    ctx.permissions().set(Permission::new(
        ns_subject.subject_id.clone(),
        ns_subject.path.clone(),
        request_dto
            .permissions
            .iter()
            .map(|p| PermissionType::from_string(p.to_string().as_str()).unwrap())
            .collect(),
    ));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/v1/acl/{path:.+}/")]
async fn get_acls(
    ns: web::Path<NamespacePath>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let ctx = new_context(con.into_inner());

    let permissions = ctx.permissions().list(ns.path.as_str())?;

    Ok(HttpResponse::Ok().json(PermissionListDto {
        permissions: permissions
            .into_iter()
            .map(|p| PermissionDto::from(p))
            .collect(),
    }))
}

#[get("/v1/acl/{path:.+}/{subject_id}")]
async fn get_acl(
    ns_subject: web::Path<NamespacedSubject>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let ctx = new_context(con.into_inner());

    let permission = ctx
        .permissions()
        .get(ns_subject.subject_id.as_str(), ns_subject.path.as_str())?;

    match permission {
        Some(p) => Ok(HttpResponse::Ok().json(PermissionDto::from(p))),
        None => Err(actix_web::error::ErrorNotFound("Not Found")),
    }
}

#[delete("/v1/acl/{path:.+}/{subject_id}")]
async fn delete_acl(
    ns_subject: web::Path<NamespacedSubject>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<auth::OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    if !auth::has_access(&user, oidc_config.as_ref(), OperationType::Admin) {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    let mut ctx = new_context(con.into_inner());

    ctx.permissions()
        .delete(ns_subject.subject_id.as_str(), ns_subject.path.as_str());

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}
