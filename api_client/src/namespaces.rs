use crate::MakoApiClient;
use crate::errors::ApiClientError;
use shared::dtos::namespaces::{NamespaceListDto, NamespacePath};
use shared::dtos::values::ValueListDto;

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

    pub async fn delete(&self, dto: NamespacePath) -> Result<(), ApiClientError> {
        let url = format!("v1/namespaces/{}", dto.path);

        let resp = self
            .client
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

    pub async fn list(&self) -> Result<NamespaceListDto, ApiClientError> {
        let url = "v1/namespaces";

        let resp = self
            .client
            .request(reqwest::Method::GET, url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api(status, text));
        }

        let namespace_list: NamespaceListDto = resp.json().await?;
        Ok(namespace_list)
    }

    pub async fn get_kvs(&self, dto: NamespacePath) -> Result<ValueListDto, ApiClientError> {
        let url = format!("v1/namespaces/{}", dto.path);

        let resp = self
            .client
            .request(reqwest::Method::GET, &url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api(status, text));
        }

        let value_list: ValueListDto = resp.json().await?;
        Ok(value_list)
    }
}
