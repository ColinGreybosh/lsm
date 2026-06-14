use std::process::ExitCode;

use clap::Parser;
use lsm::{
    Keyable, LogStructuredMergeTree,
    cli::{Cli, Commands},
    wal::message::{Key, Value},
};

fn main() -> ExitCode {
    let cli = Cli::parse();

    let mut lsm = LogStructuredMergeTree::new(&cli.path);

    match cli.command {
        Commands::Set { key, value } => match lsm.put(Key::from(&key), Value::from(&value)) {
            Ok(_) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Commands::Get { key } => match lsm.get(&Key::from(&key)) {
            Some(value) => {
                println!("{}", value);
                ExitCode::SUCCESS
            }
            None => ExitCode::FAILURE,
        },
        Commands::Del { key } => match lsm.del(Key::from(&key)) {
            Ok(_) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
        Commands::Clear {} => match lsm.clear() {
            Ok(_) => ExitCode::SUCCESS,
            Err(_) => ExitCode::FAILURE,
        },
    }
}
