use std::sync::Arc;
use dashmap::DashMap;

#[derive(Clone)]
pub struct CachedValue {
    pub value: String,
    pub version: i64,
}

pub type CacheKey = (String, String); // (path, key)

#[derive(Clone)]
pub struct Cache {
    inner: Arc<DashMap<CacheKey, CachedValue>>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, path: &str, key: &str) -> Option<CachedValue> {
        self.inner.get(&(path.to_string(), key.to_string()))
            .map(|v| v.clone())
    }

    pub fn insert(&self, path: String, key: String, value: String, ver: i64) {
        self.inner.insert((path, key), CachedValue { value, version: ver });
    }
}