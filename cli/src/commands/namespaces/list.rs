use serde::Serialize;
use tabled::builder::Builder;
use tabled::settings::Style;
use mako_client::MakoApiClient;
use crate::output::{write_output, Output};

#[derive(Serialize)]
struct NamespaceListOutput {
    #[serde(rename = "namespaces")]
    namespaces: Vec<NamespaceListOutputItem>,
}

#[derive(Serialize)]
struct NamespaceListOutputItem {
    #[serde(rename = "path")]
    path: String,
}

impl Output for NamespaceListOutput {
    fn format_plain(&self) -> String {
        let mut b = Builder::new();
        b.push_record(vec!["PATH"]);
        for ns in &self.namespaces {
            b.push_record(vec![ns.path.clone()]);
        }

        let mut table = b.build();
        table.with(Style::ascii());
        table.to_string()
    }

    fn format_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn exec(client: MakoApiClient, format: String) -> anyhow::Result<()> {
    let namespaces = client.namespaces().list().await?;
    let output = NamespaceListOutput {
        namespaces: namespaces.namespaces.into_iter().map(|ns| NamespaceListOutputItem { path: ns.path }).collect(),
    };

    write_output(&output, format);

    Ok(())
}
