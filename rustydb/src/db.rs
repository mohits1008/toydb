use std::collections::HashMap;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};

pub struct Database {
    store: HashMap<String, String>,
    wal: File,
    sst_count: usize,
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
            sst_count: 0,
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

        // Flush if too big
        if self.store.len() >= 5 {
            self.flush_to_sstable();
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        // 1. Check memory first
        if let Some(val) = self.store.get(key) {
            return Some(val.clone());
        }

        // 2. Check SSTables (latest first)
        for i in (1..=self.sst_count).rev() {
            let filename = format!("data/sst_{}.txt", i);
            if let Ok(file) = File::open(filename) {
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line.unwrap();
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if parts.len() == 2 && parts[0] == key {
                        return Some(parts[1].to_string());
                    }
                }
            }
        }

        None
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

    fn flush_to_sstable(&mut self) {
        self.sst_count += 1;

        let filename = format!("data/sst_{}.txt", self.sst_count);
        let mut file = File::create(filename).expect("Failed to create SST");

        // Step 1: sort keys
        let mut entries: Vec<_> = self.store.iter().collect();
        entries.sort_by_key(|(k, _)| *k);

        // Step 2: write to file
        for (key, value) in entries {
            let line = format!("{} {}\n", key, value);
            file.write_all(line.as_bytes()).unwrap();
        }

        file.flush().unwrap();

        // Step 3: clear memory
        self.store.clear();

        println!("Flushed to SSTable!");

        if self.sst_count >= 3 {
            self.compact();
        }
    }

    fn compact(&mut self) {
        println!("Running compaction...");

        let mut merged: HashMap<String, String> = HashMap::new();

        // Read from oldest → newest
        for i in 1..=self.sst_count {
            let filename = format!("data/sst_{}.txt", i);

            if let Ok(file) = File::open(&filename) {
                let reader = BufReader::new(file);

                for line in reader.lines() {
                    let line = line.unwrap();
                    let parts: Vec<&str> = line.split_whitespace().collect();

                    if parts.len() == 2 {
                        merged.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
            }
        }

        // Convert to sorted vector
        let mut entries: Vec<_> = merged.into_iter().collect();
        entries.sort_by_key(|(k, _)| k.clone());

        // Delete old SST files
        for i in 1..=self.sst_count {
            let filename = format!("data/sst_{}.txt", i);
            let _ = std::fs::remove_file(filename);
        }

        // Write new compacted SST
        let filename = "data/sst_1.txt";
        let mut file = File::create(filename).expect("Failed to create compacted SST");

        for (key, value) in entries {
            let line = format!("{} {}\n", key, value);
            file.write_all(line.as_bytes()).unwrap();
        }

        file.flush().unwrap();

        // Reset state
        self.sst_count = 1;

        println!("Compaction complete!");
    }
}