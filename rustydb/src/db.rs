use std::collections::HashMap;

pub struct Database {
    pub store: HashMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: HashMap::new(),
        }
    }

    // Insert a key-value pair into the database
    pub fn put(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    // Retrieve a value by key from the database
    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}