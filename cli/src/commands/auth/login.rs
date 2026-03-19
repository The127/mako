use anyhow::Result;
use mako_client::oidc::{OidcClient, PollError};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::time::{Duration, sleep};

#[derive(Serialize, Deserialize)]
pub struct StoredCredentials {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn credentials_path() -> PathBuf {
    let mut path = dirs_next::config_dir()
        .unwrap_or_else(|| PathBuf::from("."));
    path.push("mako");
    path.push("credentials.json");
    path
}

pub fn load_credentials() -> Option<StoredCredentials> {
    let path = credentials_path();
    let data = std::fs::read_to_string(&path).ok()?;
    serde_json::from_str(&data).ok()
}

pub async fn exec(issuer: String, client_id: String) -> Result<()> {
    let oidc = OidcClient::new();

    let flow = oidc.begin_device_flow(&issuer, &client_id, "openid profile email").await?;

    println!("Open this URL to activate:\n  {}", flow.verification_uri_complete);
    println!("\nOr go to {} and enter code: {}", flow.verification_uri, flow.user_code);
    println!("\nWaiting for authorization...");

    let interval = Duration::from_secs(flow.interval.max(5));

    let tokens = loop {
        sleep(interval).await;

        match oidc.poll_token(&issuer, &client_id, &flow.device_code).await {
            Ok(tokens) => break tokens,
            Err(PollError::Pending) => continue,
            Err(PollError::Denied) => anyhow::bail!("Authorization denied."),
            Err(PollError::Expired) => anyhow::bail!("Device code expired. Please try again."),
            Err(PollError::Other(e)) => return Err(e),
        }
    };

    let creds = StoredCredentials {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
    };

    let path = credentials_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(&creds)?)?;

    println!("✓ Logged in. Credentials saved to {}", path.display());

    Ok(())
}
