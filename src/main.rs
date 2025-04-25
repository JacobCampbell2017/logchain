
// main.rs
mod log;

use clap::{Parser, Subcommand};
use log::{log_entries, LogEntry, Log};

/// CLI logbook for timestamped notes and tags
#[derive(Parser)]
#[command(name = "logchain")]
#[command(version = "0.1")]
#[command(about = "A personal CLI logbook for tracking work and notes", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new log entry
    New {
        message: String,
    },

    /// List today's logs
    List,

    /// Add tags to the latest log
    Tag {
        /// One or more tags
        tags: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut logs = Log::init();
    println!("{}", logs);
    match cli.command {
        Commands::New { message } => {
            let entry = LogEntry::new(message);
            logs.add_log(entry);
            log_entries(logs).expect("Error adding log");
        }

        Commands::List => {
            println!("Listing today's logs");
            // Next step: read from file and show
        }

        Commands::Tag { tags } => {
            println!("Tagging last entry with: {:?}", tags);
            // Next step: load last entry, update tags
        }
    }
}

