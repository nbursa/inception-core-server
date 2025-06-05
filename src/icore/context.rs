use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Default)]
pub struct Context {
    data: RwLock<HashMap<String, String>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }

    pub fn set(&self, key: &str, value: &str) {
        if let Ok(mut map) = self.data.write() {
            map.insert(key.to_string(), value.to_string());
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.data.read().ok()?.get(key).cloned()
    }

    pub fn clear(&self) {
        if let Ok(mut map) = self.data.write() {
            map.clear();
        }
    }

    pub fn all(&self) -> Vec<(String, String)> {
        self.data
            .read()
            .map(|map| map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default()
    }
}
