// main.rs
mod cli_parse;
mod config;
mod log;

use cli_parse::Commands;
use log::{Log, LogEntry, log_entries};

fn main() {
    // Args

    let cli = cli_parse::parse();
    let mut logs = Log::init(None);

    match &cli.command {
        Some(Commands::Config {
            user_name,
            user_email,
        }) => {
            let config = config::set_config();
            config::modify_config(vec![user_name.clone(), user_email.clone()]);
            println!("{:?}", config);
        }
        Some(Commands::New { message, tags }) => {
            let mut entry = LogEntry::new(message.clone());
            logs.add_log(entry.clone());

            if !tags.is_none() {
                logs.add_tags(tags.clone().expect("Error adding tags"));
                entry = logs.logs.last().unwrap().clone();
            }
            log_entries(logs, None).expect("Error adding log");
            println!("New log added: {}", entry);
        }

        Some(Commands::Remove) => {
            logs.remove_log();
            log_entries(logs, None).expect("Error removing latest log");
        }
        Some(Commands::List { date }) => match date {
            Some(_) => {
                Log::init(date.clone()).display_logs();
            }
            None => logs.display_logs(),
        },

        Some(Commands::Tag { tags }) => {
            println!("Tagging last entry with: {:?}", tags);
            logs.add_tags(tags.clone());
            log_entries(logs, None).expect("Error adding tag(s)");
        }

        None => {
            println!(
                "{} version {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            );
            println!("{}", env!("CARGO_PKG_AUTHORS"));
            println!("{}\n", env!("CARGO_PKG_DESCRIPTION"));
            println!("[logchain --help] for commands")
        }
    }
}
