use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct DeviceFlowResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Deserialize)]
struct OAuthError {
    error: String,
}

#[derive(Debug, Error)]
pub enum PollError {
    #[error("authorization pending")]
    Pending,
    #[error("access denied")]
    Denied,
    #[error("token expired")]
    Expired,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub struct OidcClient {
    client: Client,
}

impl OidcClient {
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn begin_device_flow(
        &self,
        issuer: &str,
        client_id: &str,
        scope: &str,
    ) -> Result<DeviceFlowResponse> {
        let url = format!("{}/device", issuer.trim_end_matches('/'));
        let resp = self.client
            .post(&url)
            .form(&[("client_id", client_id), ("scope", scope)])
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("device flow request failed: {}", resp.status());
        }

        Ok(resp.json::<DeviceFlowResponse>().await?)
    }

    pub async fn poll_token(
        &self,
        issuer: &str,
        client_id: &str,
        device_code: &str,
    ) -> Result<TokenResponse, PollError> {
        let url = format!("{}/token", issuer.trim_end_matches('/'));
        let resp = self.client
            .post(&url)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("client_id", client_id),
                ("device_code", device_code),
            ])
            .send()
            .await
            .map_err(|e: reqwest::Error| PollError::Other(e.into()))?;

        if resp.status() == reqwest::StatusCode::BAD_REQUEST {
            let err = resp.json::<OAuthError>()
                .await
                .map_err(|e: reqwest::Error| PollError::Other(e.into()))?;
            return match err.error.as_str() {
                "authorization_pending" => Err(PollError::Pending),
                "access_denied" => Err(PollError::Denied),
                "expired_token" => Err(PollError::Expired),
                other => Err(PollError::Other(anyhow::anyhow!("oauth error: {}", other))),
            };
        }

        if !resp.status().is_success() {
            return Err(PollError::Other(anyhow::anyhow!("token request failed: {}", resp.status())));
        }

        resp.json::<TokenResponse>()
            .await
            .map_err(|e: reqwest::Error| PollError::Other(e.into()))
    }
}
