use mako_client::MakoApiClient;
use shared::dtos::namespaces::NamespacePath;

pub async fn exec(client: MakoApiClient, path: String) -> anyhow::Result<()> {
    client.namespaces().create(NamespacePath { path }).await?;
    Ok(())
}