#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NamespacePath {
    #[serde(rename = "path")]
    pub path: String,
}
