#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NamespacedKey {
    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "key")]
    pub key: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreateValueDto {
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ValueDto {
    #[serde(rename = "key")]
    pub key: String,

    #[serde(rename = "value")]
    pub value: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ValueListDto {
    #[serde(rename = "values")]
    pub values: Vec<ValueDto>,
}
