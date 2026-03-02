use crate::output::{write_output, Output};
use serde::Serialize;

#[derive(Serialize)]
struct AclListOutput {
    #[serde(rename = "acls")]
    acls: Vec<AclListOutputItem>,
}

#[derive(Serialize)]
struct AclListOutputItem {
    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "subject")]
    subject: String,

    #[serde(rename = "permissions")]
    permissions: Vec<String>,
}

impl Output for AclListOutput {
    fn format_plain(&self) -> String {
        let mut b = tabled::builder::Builder::new();
        b.push_record(vec!["PATH", "SUBJECT", "PERMISSIONS"]);
        for acl in &self.acls {
            b.push_record(vec![
                acl.path.clone(),
                acl.subject.clone(),
                acl.permissions.join(", "),
            ]);
        }

        let mut table = b.build();
        table.with(tabled::settings::Style::ascii());
        table.to_string()
    }

    fn format_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn exec(
    client: mako_client::MakoApiClient,
    path: String,
    format: String,
) -> anyhow::Result<()> {
    let response = client.acl().list(path).await?;

    let output = AclListOutput {
        acls: response
            .permissions
            .iter()
            .map(|p| AclListOutputItem {
                path: p.path.clone(),
                subject: p.subject_id.clone(),
                permissions: p.permissions.iter().map(|p| p.to_string()).collect(),
            })
            .collect(),
    };

    write_output(&output, format);
    Ok(())
}
