#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateValueDto {
    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}
