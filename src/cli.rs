use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Turn debugging information on
    pub debug: Option<bool>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Insert (or update) the value for a particular key
    Set { key: String, value: String },
}
