use std::sync::Arc;
use dashmap::DashMap;

#[derive(Clone)]
pub struct ValueCachedValue {
    pub value: String,
    pub version: i64,
}

pub type ValueCacheKey = (String, String); // (path, key)

#[derive(Clone)]
pub struct ValueCache {
    inner: Arc<DashMap<ValueCacheKey, ValueCachedValue>>,
}

impl ValueCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
        }
    }

    pub fn get(&self, path: &str, key: &str) -> Option<ValueCachedValue> {
        self.inner.get(&(path.to_string(), key.to_string()))
            .map(|v| v.clone())
    }

    pub fn insert(&self, path: String, key: String, value: String, ver: i64) {
        self.inner.insert((path, key), ValueCachedValue { value, version: ver });
    }
}