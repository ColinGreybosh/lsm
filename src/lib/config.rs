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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn should_get_base_path_from_environment() {
        let _guard = ENV_LOCK.lock().unwrap();
        let expected_path = "custom_kv_dir";
        unsafe {
            std::env::set_var("KV_DATA_DIR", expected_path);
        }
        let config = Config::parse();
        unsafe {
            std::env::remove_var("KV_DATA_DIR");
        }
        assert_eq!(config.base_path, expected_path);
    }

    #[test]
    fn should_get_default_base_path_if_no_environment_variable() {
        let _guard = ENV_LOCK.lock().unwrap();
        unsafe {
            std::env::remove_var("KV_DATA_DIR");
        }
        let config = Config::parse();
        assert_eq!(config.base_path, "wal");
    }
}
