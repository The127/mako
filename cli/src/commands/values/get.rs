use serde::Serialize;
use tabled::builder::Builder;
use mako_client::MakoApiClient;
use shared::dtos::values::NamespacedKey;
use crate::output::{write_output, Output};
use anyhow::Result;

#[derive(Serialize)]
struct GetValueOutput {
    #[serde(rename = "key")]
    key: String,

    #[serde(rename = "value")]
    value: String,

    #[serde(rename = "version")]
    version: u64,
}

impl Output for GetValueOutput {
    fn format_plain(&self) -> String {
        let mut b = Builder::new();
        b.push_record(vec!["KEY", "VALUE", "VERSION"]);
        b.push_record(vec![self.key.clone(), self.value.clone(), self.version.to_string()]);

        let mut table = b.build();
        table.with(tabled::settings::Style::ascii());
        table.to_string()
    }

    fn format_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn exec(client: MakoApiClient, path: String, key: String, format: String) -> Result<()> {
    let value = client.values().get(NamespacedKey{
        path,
        key,
    }).await?;

    let output = GetValueOutput {
        key: value.key,
        value: value.value,
        version: value.version,
    };

    write_output(&output, format);
    Ok(())
}
