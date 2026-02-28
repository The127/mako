pub mod api;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use actix_web::web::Data;
use clap::Parser;
use env_logger::Env;
use rqlite_client::{embed_migrations, Connection};
use rqlite_client::migration::Migration;

embed_migrations!(pub(crate) MyEmbeddedData("migrations"));

#[derive(Parser)]
#[command(name = "mako")]
#[command(about = "The mako kv api binary.", long_about = None)]
pub struct MakoCli {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 8080)]
    port: u16,

    #[arg(long, default_value = "http://localhost:4003")]
    database_connection: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::default()
        .filter_or("MAKO_LOG_LEVEL", "debug")
        .write_style_or("MAKO_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let cli = MakoCli::parse();

    let con = Connection::new(&cli.database_connection).unwrap();
    Migration::from_embed::<MyEmbeddedData>().migrate(&con).unwrap();

    log::info!("Starting mako on {}:{}", cli.host, cli.port);

    HttpServer::new(move|| App::new().configure(init_app).app_data(Data::new(con.clone())))
        .bind((cli.host, cli.port))?
        .run()
        .await
}

fn init_app(cfg: &mut web::ServiceConfig) {
    api::configure(cfg);
    cfg.service(health);
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
