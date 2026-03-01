use shared::dtos::namespaces::NamespacePath;

pub async fn exec(client: mako_client::MakoApiClient, path: String) -> anyhow::Result<()> {
    client.namespaces().delete(NamespacePath{
        path
    }).await?;
    Ok(())
}