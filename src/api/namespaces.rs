use actix_web::{get, post, HttpResponse, Responder};



#[get("/namespaces")]
async fn create_namespace() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}