pub mod output;
pub mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use mako_client::MakoApiClient;
use mako_client::auth::ApiTokenAuthProvider;

#[derive(Parser)]
#[command(name = "mako", version = "v0.1.0", about = "The mako kv cli binary.", long_about = None)]
struct Cli {
    #[clap(long, env = "MAKO_URL")]
    pub url: String,

    #[clap(long, short, env = "MAKO_FORMAT", default_value = "plain")]
    pub format: String,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Namespaces {
        #[clap(subcommand)]
        command: NamespaceCommands,
    },
    Kv{
        #[clap(subcommand)]
        command: KvCommands,
    },
}

#[derive(Subcommand)]
enum NamespaceCommands {
    Create { path: String },
    List,
    Delete { path: String },
    Kvs { path: String },
}

#[derive(Subcommand)]
enum KvCommands {
    Set { path: String, key: String, value: String },
    Get { path: String, key: String },
    Delete { path: String, key: String },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let env = Env::default()
        .filter_or("MAKO_LOG_LEVEL", "error")
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
            NamespaceCommands::Create { path } => commands::namespaces::create::exec(client, path).await,
            NamespaceCommands::List => commands::namespaces::list::exec(client, cli.format).await,
            NamespaceCommands::Kvs { path } => commands::namespaces::list_kvs::exec(client, path, cli.format).await,
            NamespaceCommands::Delete { path } => commands::namespaces::delete::exec(client, path).await,
        },
        Commands::Kv { command } => match command {
            KvCommands::Set { path, key, value } => commands::values::set::exec(client, path, key, value).await,
            KvCommands::Get { path, key } => commands::values::get::exec(client, path, key, cli.format).await,
            KvCommands::Delete { path, key } => commands::values::delete::exec(client, path, key).await,
        },
    }
}
