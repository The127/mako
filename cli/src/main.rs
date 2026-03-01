use clap::{Parser, Subcommand};
use env_logger::Env;
use mako_client::MakoApiClient;
use shared::dtos::namespaces::CreateNamespaceDto;
use anyhow::Result;

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
    Namespaces{
        #[clap(subcommand)]
        command: NamespaceCommands,
    },
}

#[derive(Subcommand)]
enum NamespaceCommands {
    Create {
        path: String,
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let env = Env::default()
        .filter_or("MAKO_LOG_LEVEL", "debug")
        .write_style_or("MAKO_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let cli = Cli::parse();

    let client = MakoApiClient::new(cli.url);

    match cli.command {
        Commands::Namespaces { command } => {
            match command {
                NamespaceCommands::Create { path } => {
                    create_namespace(client, path).await
                }
            }
        },
    }
}

async fn create_namespace(client: MakoApiClient, path: String) -> Result<()>{
    client.namespaces().create(CreateNamespaceDto{
        path
    }).await?;
    Ok(())
}
