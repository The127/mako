use std::fmt::format;
use reqwest::Client;
use shared::dtos::permissions::{NamespacedSubject, PermissionDto, PermissionListDto};
use crate::errors::ApiClientError;
use crate::MakoApiClient;

pub struct AclClient<'a> {
    client: &'a MakoApiClient,
}

impl<'a> AclClient<'a> {
    pub fn new(client: &'a MakoApiClient) -> Self {
        AclClient { client }
    }

    pub async fn set(&self, ns_subject: NamespacedSubject) -> Result<(), ApiClientError> {
        let url = format!("v1/acl/{}/{}", ns_subject.path, ns_subject.subject_id);

        let resp = self.client
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

    pub async fn get(&self, ns_subject: NamespacedSubject) -> Result<PermissionDto, ApiClientError> {
        let url = format!("v1/acl/{}/{}", ns_subject.path, ns_subject.subject_id);

        let resp = self.client
            .request(reqwest::Method::GET, &url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api(status, text));
        }

        let value: PermissionDto = resp.json().await?;
        Ok(value)
    }

    pub async fn delete(&self, ns_subject: NamespacedSubject) -> Result<(), ApiClientError> {
        let url = format!("v1/acl/{}/{}", ns_subject.path, ns_subject.subject_id);

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

    pub async fn list(&self, ns_subject: NamespacedSubject) -> Result<PermissionListDto, ApiClientError> {
        let url = format!("v1/acl/{}/", ns_subject.path.trim_end_matches('/'));

        let resp = self.client
            .request(reqwest::Method::GET, &url)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ApiClientError::Api(status, text));
        }

        let value: PermissionListDto = resp.json().await?;
        Ok(value)
    }
}
