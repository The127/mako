use shared::dtos::values::{CreateValueDto, NamespacedKey};

pub async fn exec(client: mako_client::MakoApiClient, path: String, key: String, value: String) -> anyhow::Result<()> {
    client.values().set(NamespacedKey{
        path,
        key,
    }, CreateValueDto{
        value,
    }).await?;
    Ok(())
}