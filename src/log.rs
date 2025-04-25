// log.rs

use chrono:: Local;
use serde::{Deserialize, Serialize};
use core::fmt;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::fs;
use std::path::PathBuf;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub time: String,
    pub message: String,
    pub tags: Vec<String>,
}

impl LogEntry {
    pub fn new(message: String) -> Self {
        Self {
            time: Local::now().time().to_string(),
            message,
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tags: Vec<String>) {
        self.tags.extend(tags)
    }
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.time, self.message, self.tags.to_string())
    }
}
/// Struct for JSON array
#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub date: String,
    pub logs: Vec<LogEntry>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            date: Local::now().date_naive().to_string(),
            logs: Vec::new(),
        }
    }

    pub fn init() -> Self {
        let mut file = File::open(Local::now().format("%Y-%m-%d").to_string()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
    pub fn add_log(&mut self, log: LogEntry) {
        self.logs.push(log);
    }
}
    
impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.date, self.logs.to_string())
    }
}


/// Returns a path like: <repo>/logs/YYYY-MM-DD.json
fn get_log_file_path() -> PathBuf {
    // Get the path to the binary
    let exe_path = std::env::current_exe().expect("Failed to get binary path");

    // Go to parent directory (likely /target/debug/)
    let base_dir = exe_path
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("Failed to find project root");

    // Append or create logs directory
    let logs_dir = base_dir.join("logs");
    fs::create_dir_all(&logs_dir).expect("Failed to create logs directory");

    // today's filename
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    logs_dir.join(format!("{}.json", date_str))
}

/// Appends log to daily log or creates new log file
pub fn log_entries(log: Log) -> Result<(), std::io::Error> {
    let mut options = OpenOptions::new();
    let file = options.create(true).truncate(true).open(get_log_file_path());
    let entry = serde_json::to_string(&log).unwrap();
    
    // append log to file
    writeln!(file.unwrap(), "{}", entry)
}
