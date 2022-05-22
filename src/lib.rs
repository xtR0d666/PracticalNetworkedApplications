use std::collections::HashMap;

/// kvstore struct
#[derive(Default)]

pub struct KvStore {
    map: HashMap<String, String>,
}
///
impl KvStore {
    /// Create a new KvStore.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the value of a string Key to a string.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Gets the string value of a given string key.
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Remove a given key.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
