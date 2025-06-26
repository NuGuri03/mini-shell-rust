use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct History {
    entries: Vec<String>,
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn push(&mut self, entry: String) {
        if !entry.trim().is_empty() {
            self.entries.push(entry);
        }
    }

    pub fn load_history(&mut self, path: &str) {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                self.entries.push(line);
            }
        }
    }

    pub fn save_history(&self, path: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
        {
            for entry in &self.entries {
                writeln!(file, "{}", entry).unwrap();
            }
        }
    }
}
