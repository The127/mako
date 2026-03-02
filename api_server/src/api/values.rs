use crate::cache::Cache;
use crate::extractors::auth::AuthUser;
use crate::repositories::rqlite::new_context;
use crate::repositories::values::Value;
use actix_web::{delete, get, put, web, HttpResponse};
use shared::dtos::values::{CreateValueDto, NamespacedKey, ValueDto};

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

    ctx.values().set(Value::new(
        ns_key.path.clone(),
        ns_key.key.clone(),
        request_dto.value.clone(),
    ));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}

#[get("/v1/kv/{path:.+}/{key}")]
async fn get_value(
    ns_key: web::Path<NamespacedKey>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    cache: web::Data<Cache>,
) -> Result<HttpResponse, actix_web::error::Error> {
    match user {
        AuthUser::Anonymous => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        AuthUser::Oidc { .. } => {
            return Err(actix_web::error::ErrorUnauthorized("Unauthorized: TODO"));
        }
        AuthUser::Admin => (),
    }

    let ctx = new_context(con.into_inner());

    if let Some(cached_value) = cache.get(&ns_key.path, &ns_key.key) {
        if let Some(db_version) = ctx.values().get_version(&ns_key.path, &ns_key.key)? {
            if cached_value.version == db_version {
                return Ok(HttpResponse::Ok().json(ValueDto{
                    key:  ns_key.key.clone(),
                    value: cached_value.value.clone(),
                    version: cached_value.version as u64,
                }));
            }
        }
    }

    let value = ctx.values().get(&ns_key.path, &ns_key.key)?;
    match value {
        Some(value) => {
            cache.insert(ns_key.path.clone(), ns_key.key.clone(), value.value(), value.version());

            Ok(HttpResponse::Ok().json(ValueDto{
                key: value.key(),
                value: value.value(),
                version: value.version() as u64,
            }))
        },
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/v1/kv/{path:.+}/{key}")]
async fn delete_value(
    ns_key: web::Path<NamespacedKey>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
) -> Result<HttpResponse, actix_web::error::Error>{
    match user {
        AuthUser::Anonymous => return Err(actix_web::error::ErrorUnauthorized("Unauthorized")),
        AuthUser::Oidc { .. } => {
            return Err(actix_web::error::ErrorUnauthorized("Unauthorized: TODO"));
        }
        AuthUser::Admin => (),
    }

    let mut ctx = new_context(con.into_inner());

    ctx.values().delete_if_exists(&ns_key.path, &ns_key.key);

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}
