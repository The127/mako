use actix_web::{post, web, HttpResponse, Responder};
use crate::repositories::namespaces::Namespace;
use crate::repositories::rqlite::new_context;
use crate::repositories::values::Value;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CreateValueDto {
    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "key")]
    key: String,

    #[serde(rename = "value")]
    value: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CreateValueQueryParams {
    #[serde(rename = "create_namespace")]
    create_namespace: Option<bool>,
}

#[post("/values")]
async fn create_value(
    request_dto: web::Json<CreateValueDto>,
    params: web::Query<CreateValueQueryParams>,
    con: web::Data<rqlite_client::Connection>,
) -> impl Responder {
    let mut ctx = new_context(con.into_inner());

    let namespace = ctx.namespaces().
        get(&request_dto.path);

    if namespace.is_none() {
        if params.create_namespace.unwrap_or(false) {
            ctx.namespaces().insert(Namespace::new(request_dto.path.clone()));
        }else{
            return HttpResponse::NotFound().body(
                format!("Namespace {} not found", request_dto.path)
            )
        }
    }

    ctx.values()
        .insert(Value::new(request_dto.path.clone(), request_dto.key.clone(), request_dto.value.clone()));

    ctx.save_changes();

    HttpResponse::NoContent().finish()
}