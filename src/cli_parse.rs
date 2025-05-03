/// cli.rs
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
    /// Modify config.toml
    Config {
        #[arg(
            short = 'u',
            long = "user.name",
            value_name = "USER_NAME",
            help = "Adds name to log Entry's - Future use for collaboritive logs"
        )]
        user_name: String,
        #[arg(
            short = 'e',
            long = "user.email",
            value_name = "EMAIL",
            help = "Adds email to log Entry's - Future use for collaboritive logs"
        )]
        user_email: String,
    },
    /// Add a new log entry
    New {
        message: String,
        #[arg(
            short = 't',
            long = "tag",
            value_name = "TAG",
            help = "Tag the log entry. Tags seperated by commas.",
            value_delimiter = ','
        )]
        tags: Option<Vec<String>>,
    },

    /// Remove latest log entry
    Remove,

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

    /// Add tags to the latest log entry
    Tag {
        /// One or more tags
        #[arg(value_delimiter = ',')]
        tags: Vec<String>,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
