use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub struct History {
    pub entries: Vec<String>,
    pub index: Option<usize>
}

impl History {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: None
        }
    }

    pub fn push(&mut self, entry: String) {
        self.entries.push(entry);
        if let Some(i) = self.index {
            self.index = Some(i + 1);
        } else {
            self.index = Some(0);
        }
    }

    pub fn load_history(&mut self, path: &str) {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            for line in reader.lines().flatten() {
                self.entries.push(line);
                if let Some(i) = self.index {
                    self.index = Some(i + 1);
                } else {
                    self.index = Some(0);
                }
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
