use clap::Parser;
use lsm::{
    Keyable, LogStructuredMergeTree,
    cli::{Cli, Commands},
    wal::message::{Key, Value},
};

fn main() {
    let cli = Cli::parse();

    let mut lsm = LogStructuredMergeTree::new();

    if cli.debug.is_some_and(|x| x) {
        println!("Initial contents: {:?}", lsm);
    }

    match cli.command {
        Commands::Set { key, value } => {
            match lsm.put(Key::from(&key), Value::from(&value)) {
                Ok(_) => println!("Success!"),
                Err(_) => panic!(),
            };
        }
    }
}
