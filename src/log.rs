/// log.rs
use chrono::Local;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::exit;
use std::{env, fs};

/// Timestamped Log Entry
#[derive(Clone, Debug, Serialize, Deserialize)]
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
/// Struct for Log
#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    /// Date in YYYY-MM-DD format
    pub date: String,
    /// Vector of LogEntry's
    pub logs: Vec<LogEntry>,
}

impl Log {
    /// Constructor that sets time to current date
    pub fn new() -> Self {
        Self {
            date: Local::now().date_naive().to_string(),
            logs: Vec::new(),
        }
    }

    /// Can use date as optional perameter for creating Log from past dates
    pub fn init(date: Option<String>) -> Self {
        let path = get_log_file_path(date.clone());
        if !path.exists() {
            match date {
                Some(_) => {
                    // FILE does not exist
                    eprintln!(
                        "File for specified date does not exist. Ensure date is in YYYY-MM-DD format."
                    );
                    exit(1);
                }
                // Create new log file for today
                None => Self::new(),
            }
        } else {
            let mut file = File::open(path).expect("Error opening file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Error reading file");
            serde_json::from_str(&contents).expect("Error parsing file")
        }
    }

    /// Push log to self.logs
    pub fn add_log(&mut self, log: LogEntry) {
        self.logs.push(log);
    }

    /// Adds tags to last LogEntry in self.logs
    pub fn add_tags(&mut self, tags: Vec<String>) {
        if let Some(last) = self.logs.last_mut() {
            last.add_tag(tags);
        } else {
            println!("[warn] No log entries yet — cannot add tags.");
        }
    }

    pub fn remove_log(&mut self) {
        self.logs.pop();
    }

    /// Displays all LogEntry in self.logs formatted to terminal
    pub fn display_logs(&self) {
        let base_dir = get_base_path();
        let width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
        let current_dir = base_dir
            .file_name()
            .and_then(|os_str| os_str.to_str())
            .unwrap_or("logchain");
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("Daily log for {}: {}", current_dir, self.date,);
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

/// Returns a PathBuf like: repo/logchain/logs/YYYY-MM-DD.json
fn get_log_file_path(date: Option<String>) -> PathBuf {
    let date_str = match date {
        Some(d) => d,
        None => Local::now().format("%Y-%m-%d").to_string(),
    };

    // Get the path to the binary
    let base_dir = get_base_path();

    // Append or create logs directory
    let logs_dir = base_dir.join("logchain").join("logs");
    fs::create_dir_all(&logs_dir).expect("Failed to create logs directory");

    // today's filename
    // let date_str = Local::now().format("%Y-%m-%d").to_string();
    logs_dir.join(format!("{}.json", date_str))
}

/// Returns PathBuf of parent Repo
fn get_base_path() -> PathBuf {
    env::current_dir().expect("Failed to get current directory")
}

/// Appends log to daily log or creates new log file
pub fn log_entries(log: Log, date: Option<String>) -> Result<(), std::io::Error> {
    let mut options = OpenOptions::new();
    let file = options
        .create(true)
        .write(true)
        .truncate(true)
        .open(get_log_file_path(date));
    let entry =
        serde_json::to_string(&log).expect("Failed to serialize log into JSON for writing to file");

    // append log to file
    writeln!(file.expect("file error?"), "{}", entry)
}
