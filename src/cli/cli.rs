use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Insert (or update) the value for a particular key
    Set { key: String, value: String },
    /// Get the value for a particular key
    Get { key: String },
    /// Delete a particular key
    Del { key: String },
    /// Delete all keys
    Clear {},
}
