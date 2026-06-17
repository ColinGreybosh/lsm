pub struct Config {
    pub base_path: String,
    pub host: String,
    pub port: String,
}

impl Config {
    pub fn parse() -> Self {
        Self {
            base_path: std::env::var("KV_DATA_DIR").unwrap_or_else(|_| "wal".to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("PORT").unwrap_or_else(|_| "50051".to_string()),
        }
    }
}
