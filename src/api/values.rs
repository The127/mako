use crate::repositories::rqlite::new_context;
use crate::repositories::values::Value;
use actix_web::{post, web, HttpResponse};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CreateValueDto {
    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "key")]
    key: String,

    #[serde(rename = "value")]
    value: String,
}

#[post("/values")]
async fn create_value(
    request_dto: web::Json<CreateValueDto>,
    con: web::Data<rqlite_client::Connection>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let mut ctx = new_context(con.into_inner());

    ctx.values()
        .insert(Value::new(request_dto.path.clone(), request_dto.key.clone(), request_dto.value.clone()));

    ctx.save_changes()?;

    Ok(HttpResponse::NoContent().finish())
}