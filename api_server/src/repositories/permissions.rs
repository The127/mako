#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PermissionType {
    Read,
    Write,
}

impl std::fmt::Display for PermissionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionType::Read => write!(f, "read"),
            PermissionType::Write => write!(f, "write"),
        }
    }
}

impl PermissionType {
    pub fn from_string(s: &str) -> Option<PermissionType> {
        match s {
            "read" => Some(PermissionType::Read),
            "write" => Some(PermissionType::Write),
            _ => None,
        }
    }   
}

pub struct Permission {
    subject_id: String,
    path: String,
    permissions: Vec<PermissionType>,
}

impl Permission {
    pub fn new(subject_id: String, path: String, permissions: Vec<PermissionType>) -> Self {
        Permission { subject_id, path, permissions }
    }

    pub fn subject_id(&self) -> String {
        self.subject_id.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn permissions(&self) -> Vec<PermissionType> {
        self.permissions.clone()
    }
    
    pub fn has_permission(&self, permission: PermissionType) -> bool {
        self.permissions.contains(&permission)
    }  
}

pub trait PermissionRepository {
    fn set(&self, permission: Permission);
    fn get(&self, subject_id: &str, path: &str) -> Result<Option<Permission>, Box<dyn std::error::Error>>;
    fn list(&self, path: &str) -> Result<Vec<Permission>, Box<dyn std::error::Error>>;
    fn delete(&self, subject_id: &str, path: &str);
}
