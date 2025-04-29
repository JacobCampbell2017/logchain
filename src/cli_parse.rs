// cli.rs

use clap::{Parser, Subcommand, command};

/// CLI logbook for timestamped notes and tags
#[derive(Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new log entry
    New { message: String },

    /// List logs of specificied date.
    List {
        #[arg(
            short = 'd',
            long = "date",
            value_name = "DATE",
            help = "Specify a date in format of YYYY-MM-DD (default today)"
        )]
        date: Option<String>,
    },

    /// Add tags to the latest log
    Tag {
        /// One or more tags
        tags: Vec<String>,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
