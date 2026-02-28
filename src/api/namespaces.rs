use actix_web::{get, post, web, HttpResponse, Responder};
use rqlite_client::ureq::serde;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct CreateNamespaceDto {
    #[serde(rename = "path")]
    path: String,
}

#[post("/namespaces")]
async fn create_namespace(request_dto: web::Json<CreateNamespaceDto>) -> impl Responder {
    log::info!("{:?}", request_dto);
    HttpResponse::Ok().body("Hello world!")
}

#[get("/namespaces")]
async fn list_namespaces() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
