use shared::dtos::values::NamespacedKey;

pub async fn exec(client: mako_client::MakoApiClient, path: String, key: String) -> anyhow::Result<()> {
    client.values().delete(NamespacedKey{
        path,
        key,
    }).await?;
    Ok(())
}