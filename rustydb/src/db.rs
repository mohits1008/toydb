use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};

pub struct Database {
    store: HashMap<String, String>,
    wal: File,
}

impl Database {
    pub fn new() -> Self {
        // Open WAL file (create if not exists)
        let wal = OpenOptions::new()
            .create(true)
            .append(true)
            .open("data/wal.log")
            .expect("Failed to open WAL");

        let mut db = Database {
            store: HashMap::new(),
            wal,
        };

        // Recover from WAL
        db.load();

        db
    }

    pub fn put(&mut self, key: String, value: String) {
        // 1. Write to WAL first (IMPORTANT)
        let log = format!("put {} {}\n", key, value);
        self.wal.write_all(log.as_bytes()).unwrap();
        self.wal.flush().unwrap();

        // 2. Then update memory
        self.store.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    fn load(&mut self) {
        let file = File::open("data/wal.log").unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() == 3 && parts[0] == "put" {
                self.store
                    .insert(parts[1].to_string(), parts[2].to_string());
            }
        }
    }
}