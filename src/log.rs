// log.rs

use chrono::Local;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub time: String,
    pub message: String,
    pub tags: Vec<String>,
}

impl LogEntry {
    pub fn new(message: String) -> Self {
        Self {
            time: Local::now().time().format("%H:%M:%S").to_string(),
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
        if self.tags.is_empty() {
            write!(f, "[{}] {}", self.time, self.message)
        } else {
            write!(f, "[{}] {} {:?}", self.time, self.message, self.tags)
        }
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
        let path = get_log_file_path();
        if !path.exists() {
            Self::new()
        } else {
            let mut file = File::open(path).expect("Error opening file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            serde_json::from_str(&contents).unwrap()
        }
    }
    pub fn add_log(&mut self, log: LogEntry) {
        self.logs.push(log);
    }

    pub fn add_tags(&mut self, tags: Vec<String>) {
        if let Some(last) = self.logs.last_mut() {
            last.add_tag(tags);
        } else {
            println!("[warn] No log entries yet — cannot add tags.");
        }
    }

    /// Displays Log of the day
    pub fn display_logs(&self) {
        let base_dir = get_base_path();
        let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
        let current_dir = base_dir
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("logchain");
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!(
            "Daily log for {}: {}",
            current_dir,
            Local::now().format("%Y-%m-%d")
        );
        println!("{:-<1$}", "", width);
        for line in &self.logs {
            println!("{}", line);
        }
    }
}

impl fmt::Display for Log {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {:?})", self.date, self.logs)
    }
}

/// Returns a PathBuf like: <repo>/logs/YYYY-MM-DD.json
fn get_log_file_path() -> PathBuf {
    // Get the path to the binary
    let base_dir = get_base_path();

    // Append or create logs directory
    let logs_dir = base_dir.join("logs");
    fs::create_dir_all(&logs_dir).expect("Failed to create logs directory");

    // today's filename
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    logs_dir.join(format!("{}.json", date_str))
}

/// Returns PathBuf of parent Repo
fn get_base_path() -> PathBuf {
    env::current_dir()
        .expect("Failed to get current directory")
        .join("logchain")
}

/// Appends log to daily log or creates new log file
pub fn log_entries(log: Log) -> Result<(), std::io::Error> {
    let mut options = OpenOptions::new();
    let file = options
        .create(true)
        .write(true)
        .truncate(true)
        .open(get_log_file_path());
    let entry =
        serde_json::to_string(&log).expect("Failed to serialize log into JSON for writing to file");

    // append log to file
    writeln!(file.expect("file error?"), "{}", entry)
}
