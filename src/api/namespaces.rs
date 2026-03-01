use crate::repositories::namespaces::Namespace;
use crate::repositories::rqlite::new_context;
use actix_web::{get, post, web, HttpResponse, Responder};
use rqlite_client::ureq::serde;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CreateNamespaceDto {
    #[serde(rename = "path")]
    path: String,
}

#[post("/namespaces")]
async fn create_namespace(
    request_dto: web::Json<CreateNamespaceDto>,
    con: web::Data<rqlite_client::Connection>,
) -> Result<HttpResponse, actix_web::error::Error> {
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
