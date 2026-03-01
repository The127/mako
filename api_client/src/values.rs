use shared::dtos::values::CreateValueDto;
use crate::errors::ApiClientError;
use crate::MakoApiClient;

pub struct ValueClient<'a> {
    client: &'a MakoApiClient,
}

impl<'a> ValueClient<'a> {
    pub fn new(client: &'a MakoApiClient) -> Self {
        ValueClient { client }
    }

    pub async fn create(&self, dto: CreateValueDto) -> Result<(), ApiClientError> {
        let url = "values";

        let resp = self.client
            .request(reqwest::Method::POST, &url)
            .json(&dto)
            .send()
            .await?;

        if !resp.status().is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api(text));
        }

        Ok(())
    }
}
