use shared::dtos::namespaces::CreateNamespaceDto;
use crate::errors::ApiClientError;
use crate::MakoApiClient;

pub struct NamespaceClient<'a> {
    client: &'a MakoApiClient,
}

impl<'a> NamespaceClient<'a> {
    pub fn new(client: &'a MakoApiClient) -> Self {
        NamespaceClient { client }
    }

    pub async fn create(&self, dto: CreateNamespaceDto) -> Result<(), ApiClientError> {
        let url = "namespaces";

        let resp = self.client
            .request(reqwest::Method::POST, &url)
            .json(&dto)
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