pub struct Config {
    pub base_path: String,
}

impl Config {
    #[must_use]
    pub fn parse() -> Self {
        let base_path = match std::env::var("KV_DATA_DIR") {
            Ok(path) => path,
            Err(_) => String::from("wal"),
        };
        Self { base_path }
    }
}
