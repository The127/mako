use shared::dtos::values::{CreateValueDto, NamespacedKey};
use crate::errors::ApiClientError;
use crate::MakoApiClient;

pub struct ValueClient<'a> {
    client: &'a MakoApiClient,
}

impl<'a> ValueClient<'a> {
    pub fn new(client: &'a MakoApiClient) -> Self {
        ValueClient { client }
    }

    pub async fn create(&self, ns_key: NamespacedKey, dto: CreateValueDto) -> Result<(), ApiClientError> {
        let url = format!("values/{}/{}", ns_key.path, ns_key.key);

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
