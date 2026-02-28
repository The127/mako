use actix_web::{get, post, web, HttpResponse, Responder};
use rqlite_client::{response, Mapping};
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
) -> impl Responder {
    let query = con.execute()
        .push_sql_values(&["insert into namespaces(path) values (?)", request_dto.path.as_str()]);

    let response_result = response::query::Query::from(query.request_run().unwrap());

    if let Some(Mapping::Standard(success)) = response_result.results().next() {
        let row = 0;
        let col = 0;
        if let Some(rows_found) = &success.value(row, col) {
            log::info!("Rows found: {}", rows_found);
        }
    }else if let Some(Mapping::Error(error)) = response_result.results().next() {
        log::error!("Error creating namespace: {}", error);
    }

    HttpResponse::NoContent().finish()
}

#[get("/namespaces")]
async fn list_namespaces() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
