use serde::Serialize;
use tabled::builder::Builder;
use mako_client::MakoApiClient;
use shared::dtos::namespaces::NamespacePath;
use crate::output::{write_output, Output};

#[derive(Serialize)]
struct ValueListOutput {
    #[serde(rename = "values")]
    values: Vec<ValueListOutputItem>,
}

#[derive(Serialize)]
struct ValueListOutputItem {
    #[serde(rename = "path")]
    key: String,

    #[serde(rename = "value")]
    value: String,
}

impl Output for ValueListOutput {
    fn format_plain(&self) -> String {
        let mut b = Builder::new();
        b.push_record(vec!["PATH", "VALUE"]);
        for ns in &self.values {
            b.push_record(vec![ns.key.clone(), ns.value.clone()]);
        }

        let mut table = b.build();
        table.with(tabled::settings::Style::ascii());
        table.to_string()
    }

    fn format_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn exec(client: MakoApiClient, path: String, format: String) -> anyhow::Result<()> {
    let kvs = client.namespaces().get_kvs(NamespacePath { path }).await?;

    let output = ValueListOutput {
        values: kvs.values.into_iter().map(|kv| ValueListOutputItem { key: kv.key, value: kv.value }).collect(),
    };

    write_output(&output, format);
    Ok(())
}
