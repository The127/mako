pub struct Namespace {
    path: String,
}

impl Namespace {
    pub fn new(path: String) -> Self {
        Namespace { path }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }
}

pub trait NamespaceRepository {
    fn insert_if_not_exists(&self, namespace: Namespace);
    fn get(&self, path: &str) -> Result<Option<Namespace>, Box<dyn std::error::Error>>;
    fn exists(&self, path: &str) -> Result<bool, Box<dyn std::error::Error>>;
    fn delete_if_exists(&self, path: &str);  
}
