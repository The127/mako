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
    fn insert(&self, namespace: Namespace);
    fn get(&self, path: &str) -> Option<Namespace>;
}
