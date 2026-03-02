use std::fmt::Display;
use std::str::FromStr;

#[derive(serde::Deserialize, serde::Serialize, Debug, Copy, Clone)]
pub enum PermissionType {
    Read,
    Write,
}

impl Display for PermissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PermissionType::Read => "read".to_string(),
            PermissionType::Write => "write".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl FromStr for PermissionType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read" => Ok(PermissionType::Read),
            "write" => Ok(PermissionType::Write),
            _ => Err(format!("Invalid permission type: {}", s)),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CreatePermissionDto {
    #[serde(rename = "permissions")]
    pub permissions: Vec<PermissionType>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PermissionListDto {
    #[serde(rename = "permissions")]
    pub permissions: Vec<PermissionDto>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PermissionDto {
    #[serde(rename = "subjectId")]
    pub subject_id: String,

    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "permissions")]
    pub permissions: Vec<PermissionType>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NamespacedSubject {
    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "subject_id")]
    pub subject_id: String,
}
