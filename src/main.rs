// main.rs
mod log;

use clap::{Parser, Subcommand, command};
use log::{Log, LogEntry, log_entries};

/// CLI logbook for timestamped notes and tags
#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new log entry
    New { message: String },

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

    match &cli.command {
        Some(Commands::New { message }) => {
            let entry = LogEntry::new(message.clone());
            logs.add_log(entry);
            log_entries(logs).expect("Error adding log");
        }

        Some(Commands::List) => {
            logs.display_logs();
        }

        Some(Commands::Tag { tags }) => {
            println!("Tagging last entry with: {:?}", tags);
            logs.add_tags(tags.clone());
            log_entries(logs).expect("Error adding tag(s)");
        }

        None => {
            println!("Logchain version {}", env!("CARGO_PKG_VERSION"));
            println!("Author: Jacob Campbell - Jacobrcampbell20@gmail.com");
            println!("Offline-first CLI logbook. Track daily notes, commits, and progress easily.\n");
            println!("[logchain --help] for commands")
        }
    }
}
