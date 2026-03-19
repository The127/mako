use crate::cache::ValueCache;
use crate::auth::AuthUser;
use crate::repositories::rqlite::new_context;
use crate::repositories::values::Value;
use actix_web::http::header::IF_NONE_MATCH;
use actix_web::{HttpRequest, HttpResponse, delete, get, put, web};
use shared::dtos::values::{CreateValueDto, NamespacedKey, ValueDto};
use crate::auth;
use crate::auth::{OidcConfiguration, OperationType};

#[put("/v1/kv/{path:.+}/{key}")]
async fn set_value(
    ns_key: web::Path<NamespacedKey>,
    request_dto: web::Json<CreateValueDto>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut ctx = new_context(con.into_inner());

    if !auth::has_access_to_value(ctx.as_ref(), &ns_key.path, &user, oidc_config.as_ref(), OperationType::Write)? {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

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
    req: HttpRequest,
    ns_key: web::Path<NamespacedKey>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    cache: web::Data<ValueCache>,
    oidc_config: web::Data<OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let ctx = new_context(con.into_inner());

    if !auth::has_access_to_value(ctx.as_ref(), &ns_key.path, &user, oidc_config.as_ref(), OperationType::Read)? {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };

    if let Some(cached_value) = cache.get(&ns_key.path, &ns_key.key)
        && let Some(db_version) = ctx.values().get_version(&ns_key.path, &ns_key.key)?
        && cached_value.version == db_version
    {
        let etag_value = format!("\"{}\"", cached_value.version);
        if let Some(etag) = req.headers().get(IF_NONE_MATCH)
            && etag.to_str().ok() == Some(&etag_value)
        {
            return Ok(HttpResponse::NotModified().finish());
        }

        return Ok(HttpResponse::Ok()
            .insert_header((actix_web::http::header::ETAG, etag_value))
            .json(ValueDto {
                key: ns_key.key.clone(),
                value: cached_value.value.clone(),
                version: cached_value.version as u64,
            }));
    }

    let value = ctx.values().get(&ns_key.path, &ns_key.key)?;
    match value {
        Some(value) => {
            cache.insert(
                ns_key.path.clone(),
                ns_key.key.clone(),
                value.value(),
                value.version(),
            );

            let etag_version = format!("\"{}\"", value.version());
            if let Some(etag) = req.headers().get(IF_NONE_MATCH)
                && etag.to_str().ok() == Some(&etag_version)
            {
                return Ok(HttpResponse::NotModified().finish());
            }

            Ok(HttpResponse::Ok()
                .insert_header((actix_web::http::header::ETAG, etag_version))
                .json(ValueDto {
                    key: value.key(),
                    value: value.value(),
                    version: value.version() as u64,
                }))
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[delete("/v1/kv/{path:.+}/{key}")]
async fn delete_value(
    ns_key: web::Path<NamespacedKey>,
    con: web::Data<rqlite_client::Connection>,
    user: AuthUser,
    oidc_config: web::Data<OidcConfiguration>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut ctx = new_context(con.into_inner());

    if !auth::has_access_to_value(ctx.as_ref(), &ns_key.path, &user, oidc_config.as_ref(), OperationType::Write)? {
        return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
    };


    ctx.values().delete_if_exists(&ns_key.path, &ns_key.key);

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}
