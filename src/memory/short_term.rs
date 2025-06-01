use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct ShortTermMemory {
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl ShortTermMemory {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.read().ok()?.get(key).cloned()
    }

    pub fn set(&self, key: String, value: String) {
        if let Ok(mut map) = self.inner.write() {
            map.insert(key, value);
        }
    }

    pub fn delete(&self, key: &str) {
        if let Ok(mut map) = self.inner.write() {
            map.remove(key);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut map) = self.inner.write() {
            map.clear();
        }
    }

    pub fn all(&self) -> Option<HashMap<String, String>> {
        self.inner.read().ok().map(|map| map.clone())
    }
}
