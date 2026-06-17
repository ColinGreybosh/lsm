use std::env;

pub struct Config {
    pub base_path: String,
}

impl Config {
    pub fn parse() -> Self {
        let base_path = match env::var("KV_DATA_DIR") {
            Ok(path) => path,
            Err(_) => String::from("wal"),
        };
        Self { base_path }
    }
}
