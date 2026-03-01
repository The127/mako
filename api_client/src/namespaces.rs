use crate::errors::ApiClientError;
use crate::MakoApiClient;
use shared::dtos::namespaces::NamespacePath;

pub struct NamespaceClient<'a> {
    client: &'a MakoApiClient,
}

impl<'a> NamespaceClient<'a> {
    pub fn new(client: &'a MakoApiClient) -> Self {
        NamespaceClient { client }
    }

    pub async fn create(&self, dto: NamespacePath) -> Result<(), ApiClientError> {
        let url = format!("v1/namespaces/{}", dto.path);

        let resp = self
            .client
            .request(reqwest::Method::PUT, &url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api(status, text));
        }

        Ok(())
    }
}
