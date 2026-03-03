pub mod api;
pub mod cache;
pub mod repositories;
pub mod auth;

use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use clap::Parser;
use env_logger::Env;
use rqlite_client::migration::Migration;
use rqlite_client::{Connection, embed_migrations};
use crate::auth::OidcConfiguration;

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

    #[arg(long, default_value = "mako:admin", env = "MAKO_ADMIN_ROLE")]
    admin_role: String,

    #[arg(long, default_value = "mako:writer", env = "MAKO_WRITER_ROLE")]
    writer_role: String,

    #[arg(long, default_value = "mako:reader", env = "MAKO_READER_ROLE")]
    reader_role: String,

    #[arg(long, env = "MAKO_ISSUER")]
    issuer: String,

    #[arg(long, env = "MAKO_CLIENT_ID")]
    client_id: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env = Env::default()
        .filter_or("MAKO_LOG_LEVEL", "error")
        .write_style_or("MAKO_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let cli = MakoCli::parse();

    let con = Connection::new(&cli.database_connection).unwrap();
    Migration::from_embed::<MyEmbeddedData>()
        .migrate(&con)
        .unwrap();

    log::info!("Starting mako on {}:{}", cli.host, cli.port);

    HttpServer::new(move || {
        App::new()
            .configure(init_app)
            .app_data(Data::new(con.clone()))
            .app_data(Data::new(cache::ValueCache::new()))
            .app_data(Data::new(cache::JwksCache::new()))
            .app_data(Data::new(OidcConfiguration{
                admin_role: cli.admin_role.clone(),
                writer_role: cli.writer_role.clone(),
                reader_role: cli.reader_role.clone(),
                issuer: cli.issuer.clone(),
                client_id: cli.client_id.clone(),
            }))
    })
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
