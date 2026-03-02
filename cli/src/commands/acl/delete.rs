use shared::dtos::permissions::NamespacedSubject;

pub async fn exec(client: mako_client::MakoApiClient, path: String, subject: String) -> anyhow::Result<()> {
    client.acl().delete(NamespacedSubject{
        path,
        subject_id: subject,
    }).await?;
    Ok(())
}