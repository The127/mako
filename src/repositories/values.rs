pub struct Value {
    path: String,
    key: String,
    value: String,
}

impl Value {
    pub fn new(path: String, key: String, value: String) -> Self {
        Self { path, key, value }
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
}

pub trait ValueRepository {
    fn insert(&self, value: Value);
}