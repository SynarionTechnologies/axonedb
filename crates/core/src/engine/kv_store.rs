use std::collections::HashMap;
use std::time::Duration;

use crate::models::value::Value;

/// In-memory key-value store with optional per-key TTL.
#[derive(Default)]
pub struct KvStore {
    map: HashMap<Vec<u8>, Value>,
}

impl KvStore {
    /// Create a new empty [`KvStore`].
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Insert a key-value pair with an optional TTL.
    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>, ttl: Option<Duration>) {
        let value = Value::new(value, ttl);
        self.map.insert(key, value);
    }

    /// Get the value for `key`, purging it if expired.
    pub fn get(&mut self, key: &[u8]) -> Option<Vec<u8>> {
        if let Some(val) = self.map.get(key) {
            if val.is_expired() {
                self.map.remove(key);
                None
            } else {
                Some(val.data.clone())
            }
        } else {
            None
        }
    }

    /// Delete `key` from the store. Returns true if the key existed and was not expired.
    pub fn delete(&mut self, key: &[u8]) -> bool {
        if let Some(val) = self.map.get(key) {
            if val.is_expired() {
                self.map.remove(key);
                return false;
            } else {
                self.map.remove(key);
                return true;
            }
        }
        false
    }
}
