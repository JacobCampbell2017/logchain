// log.rs

use chrono:: {DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::{PathBuf};



#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Local>,
    pub message: String,
    pub tags: Vec<String>,
}

impl LogEntry {
    pub fn new(message: String) -> Self {
        Self {
            timestamp: Local::now(),
            message,
            tags: Vec::new(),
        }
    }

    pub fn add_tag(&mut self, tags: Vec<String>) {
        self.tags.extend(tags)
    }
}

/// Returns a path like: <repo>/logs/YYYY-MM-DD.json
pub fn get_log_file_path() -> PathBuf {
    // Get the path to the binary
    let exe_path = std::env::current_exe().expect("Failed to get binary path");

    // Go to parent directory (likely /target/debug/)
    let base_dir = exe_path
        .parent()
        .and_then(|p| p.parent()) // go up one more level to the project root
        .expect("Failed to find project root");

    // Append or create logs directory
    let logs_dir = base_dir.join("logs");
    fs::create_dir_all(&logs_dir).expect("Failed to create logs directory");

    // today's filename
    let date_str = Local::now().format("%Y-%m-%d").to_string();
    let file_path = logs_dir.join(format!("{}.json", date_str));

    file_path
}

