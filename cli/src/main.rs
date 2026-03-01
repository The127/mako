use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use mako_client::MakoApiClient;
use mako_client::auth::ApiTokenAuthProvider;
use shared::dtos::namespaces::NamespacePath;

#[derive(Parser)]
#[command(name = "mako", version = "v0.1.0", about = "The mako kv cli binary.", long_about = None)]
struct Cli {
    #[clap(long, env = "MAKO_URL")]
    pub url: String,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Namespaces {
        #[clap(subcommand)]
        command: NamespaceCommands,
    },
}

#[derive(Subcommand)]
enum NamespaceCommands {
    Create { path: String },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let env = Env::default()
        .filter_or("MAKO_LOG_LEVEL", "debug")
        .write_style_or("MAKO_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let cli = Cli::parse();

    let admin_token = std::env::var("MAKO_ADMIN_TOKEN");
    let client = match admin_token {
        Ok(token) => MakoApiClient::new(cli.url, Box::new(ApiTokenAuthProvider::new(token))),
        Err(_) => {
            log::error!("No admin token found, set MAKO_ADMIN_TOKEN env variable");
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Namespaces { command } => match command {
            NamespaceCommands::Create { path } => create_namespace(client, path).await,
        },
    }
}

async fn create_namespace(client: MakoApiClient, path: String) -> Result<()> {
    client.namespaces().create(NamespacePath { path }).await?;
    Ok(())
}
