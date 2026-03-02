pub struct Value {
    path: String,
    key: String,
    value: String,
    version: i64,
}

impl Value {
    pub fn new(path: String, key: String, value: String) -> Self {
        Self { path, key, value, version: 0 }
    }

    pub fn new_with_version(path: String, key: String, value: String, version: i64) -> Self {
        Self { path, key, value, version }
    }   

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn key(&self) -> String {
        self.key.clone()
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn version(&self) -> i64 {
        self.version
    }
}

pub trait ValueRepository {
    fn set(&self, value: Value);
    fn get(&self, path: &str, key: &str) -> Result<Option<Value>, Box<dyn std::error::Error>>;
    fn get_version(&self, path: &str, key: &str) -> Result<Option<i64>, Box<dyn std::error::Error>>;   
    fn list(&self, path: &str) -> Result<Vec<Value>, Box<dyn std::error::Error>>;
    fn delete_if_exists(&self, path: &str, key: &str);
}