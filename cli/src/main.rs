pub mod commands;
pub mod output;

use anyhow::Result;
use clap::{Parser, Subcommand};
use env_logger::Env;
use mako_client::MakoApiClient;
use mako_client::auth::ApiTokenAuthProvider;
use shared::dtos::permissions::PermissionType;

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
    Kv {
        #[clap(subcommand)]
        command: KvCommands,
    },
    Acl {
        #[clap(subcommand)]
        command: AclCommands,
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
    Set {
        path: String,
        key: String,
        value: String,
    },
    Get {
        path: String,
        key: String,
    },
    Delete {
        path: String,
        key: String,
    },
}

#[derive(Subcommand)]
enum AclCommands {
    Set {
        path: String,
        subject: String,
        permissions: Vec<PermissionType>,
    },
    Get {
        path: String,
        subject: String,
    },
    Delete {
        path: String,
        subject: String,
    },
    List {
        path: String,
    },
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
            NamespaceCommands::Create { path } => {
                commands::namespaces::create::exec(client, path).await
            },
            NamespaceCommands::List => {
                commands::namespaces::list::exec(client, cli.format).await
            },
            NamespaceCommands::Kvs { path } => {
                commands::namespaces::list_kvs::exec(client, path, cli.format).await
            },
            NamespaceCommands::Delete { path } => {
                commands::namespaces::delete::exec(client, path).await
            },
        },
        Commands::Kv { command } => match command {
            KvCommands::Set { path, key, value } => {
                commands::values::set::exec(client, path, key, value).await
            },
            KvCommands::Get { path, key } => {
                commands::values::get::exec(client, path, key, cli.format).await
            },
            KvCommands::Delete { path, key } => {
                commands::values::delete::exec(client, path, key).await
            },
        },
        Commands::Acl { command } => match command {
            AclCommands::Get {subject, path} => {
                commands::acl::get::exec(client, path, subject, cli.format).await
            },
            AclCommands::Delete {subject, path} => {
                commands::acl::delete::exec(client, path, subject).await
            },
            AclCommands::List {path} => {
                commands::acl::list::exec(client, path, cli.format).await
            },
            AclCommands::Set {path, subject, permissions} => {
                commands::acl::set::exec(client, path, subject, permissions).await
            },
        }
    }
}
