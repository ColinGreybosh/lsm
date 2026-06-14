use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Path to the Write-Ahead Log (WAL) directory
    #[arg(short, long, default_value = "wal")]
    pub path: std::path::PathBuf,

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
}
