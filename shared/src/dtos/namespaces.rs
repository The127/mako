#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NamespacePath {
    #[serde(rename = "path")]
    pub path: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NamespaceListDto {
    #[serde(rename = "namespaces")]
    pub namespaces: Vec<NamespaceDto>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NamespaceDto {
    #[serde(rename = "path")]
    pub path: String,
}
