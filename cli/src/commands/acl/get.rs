use serde::Serialize;
use tabled::builder::Builder;
use shared::dtos::permissions::NamespacedSubject;
use crate::output::{write_output, Output};

#[derive(Serialize)]
struct GetAclOutput {
    #[serde(rename = "path")]
    path: String,

    #[serde(rename = "subject")]
    subject: String,

    #[serde(rename = "permissions")]
    permissions: Vec<String>,
}

impl Output for GetAclOutput {
    fn format_plain(&self) -> String {
        let mut b = Builder::new();
        b.push_record(vec!["PATH", "SUBJECT", "PERMISSIONS"]);
        b.push_record(vec![self.path.clone(), self.subject.clone(), self.permissions.join(", ")]);

        let mut table = b.build();
        table.with(tabled::settings::Style::ascii());
        table.to_string()
    }

    fn format_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub async fn exec(client: mako_client::MakoApiClient, path: String, subject: String, format: String) -> anyhow::Result<()> {
    let acl = client.acl().get(NamespacedSubject{
        path,
        subject_id: subject,
    }).await?;

    let output = GetAclOutput {
        path: acl.path,
        subject: acl.subject_id,
        permissions: acl.permissions.iter().map(|p| p.to_string()).collect(),
    };

    write_output(&output, format);
    Ok(())
}
