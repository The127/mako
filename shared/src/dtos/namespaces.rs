#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateNamespaceDto {
    #[serde(rename = "path")]
    pub path: String,
}
