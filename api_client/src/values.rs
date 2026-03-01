use shared::dtos::values::{CreateValueDto, NamespacedKey, ValueDto};
use crate::errors::ApiClientError;
use crate::MakoApiClient;

pub struct ValueClient<'a> {
    client: &'a MakoApiClient,
}

impl<'a> ValueClient<'a> {
    pub fn new(client: &'a MakoApiClient) -> Self {
        ValueClient { client }
    }

    pub async fn set(&self, ns_key: NamespacedKey, dto: CreateValueDto) -> Result<(), ApiClientError> {
        let url = format!("v1/kv/{}/{}", ns_key.path, ns_key.key);

        let resp = self.client
            .request(reqwest::Method::PUT, &url)
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

    pub async fn get(&self, ns_key: NamespacedKey) -> Result<ValueDto, ApiClientError> {
        let url = format!("v1/kv/{}/{}", ns_key.path, ns_key.key);
        let resp = self.client
            .request(reqwest::Method::GET, &url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api(status, text));
        }

        let value: ValueDto = resp.json().await?;
        Ok(value)
    }

    pub async fn delete(&self, ns_key: NamespacedKey) -> Result<(), ApiClientError> {
        let url = format!("v1/kv/{}/{}", ns_key.path, ns_key.key);
        let resp = self.client
            .request(reqwest::Method::DELETE, &url)
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
