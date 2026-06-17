use clap::{Parser, Subcommand};
use kv_store_lib::{
    Keyable, LogStructuredMergeTree,
    config::Config,
    wal::message::{Key, Value},
};
use std::process::ExitCode;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Insert (or update) the value for a particular key
    Set { key: String, value: String },
    /// Get the value for a particular key
    Get { key: String },
    /// Delete a particular key
    Del { key: String },
    /// Delete all keys
    Clear {},
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = Config::parse();

    let mut lsm = LogStructuredMergeTree::new(&config.base_path);

    match cli.command {
        Commands::Set { key, value } => match lsm.put(Key::from(&key), Value::from(&value)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Commands::Get { key } => match lsm.get(&Key::from(&key)) {
            Some(value) => {
                println!("{value}");
                ExitCode::SUCCESS
            }
            None => ExitCode::FAILURE,
        },
        Commands::Del { key } => match lsm.del(Key::from(&key)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Commands::Clear {} => match lsm.clear() {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
    }
}
