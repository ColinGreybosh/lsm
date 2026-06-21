use crate::{
    cli::{Cli, Command},
    config::Config,
};
use clap::Parser;
use kv_store_lib::{
    Keyable, LogStructuredMergeTree,
    wal::message::{Key, Value},
};
use std::process::ExitCode;

mod cli;
mod config;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let config = Config::parse();

    let mut lsm = LogStructuredMergeTree::new(&config.base_path);

    match cli.command {
        Command::Set { key, value } => match lsm.put(Key::from(&key), Value::from(&value)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Command::Get { key } => match lsm.get(&Key::from(&key)) {
            Some(value) => {
                println!("{value}");
                ExitCode::SUCCESS
            }
            None => ExitCode::FAILURE,
        },
        Command::Del { key } => match lsm.del(Key::from(&key)) {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Command::Clear {} => match lsm.clear() {
            Ok(()) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
    }
}
