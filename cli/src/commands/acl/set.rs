use shared::dtos::permissions::{CreatePermissionDto, NamespacedSubject, PermissionType};

pub async fn exec(
    client: mako_client::MakoApiClient,
    path: String,
    subject: String,
    permissions: Vec<PermissionType>,
) -> anyhow::Result<()> {
    client.acl().set(NamespacedSubject{
        path,
        subject_id: subject,
    }, CreatePermissionDto{
        permissions,
    }).await?;
    Ok(())
}
