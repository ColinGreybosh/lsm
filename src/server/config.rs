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

    #[test]
    fn should_get_host_from_environment() {
        let _guard = ENV_LOCK.lock().unwrap();
        let expected_host = "custom_host";
        unsafe {
            std::env::set_var("HOST", expected_host);
        }
        let config = Config::parse();
        unsafe {
            std::env::remove_var("HOST");
        }
        assert_eq!(config.host, expected_host);
    }

    #[test]
    fn should_get_default_host_if_no_environment_variable() {
        let _guard = ENV_LOCK.lock().unwrap();
        unsafe {
            std::env::remove_var("HOST");
        }
        let config = Config::parse();
        assert_eq!(config.host, "127.0.0.1");
    }

    #[test]
    fn should_get_port_from_environment() {
        let _guard = ENV_LOCK.lock().unwrap();
        let expected_port = "custom_port";
        unsafe {
            std::env::set_var("PORT", expected_port);
        }
        let config = Config::parse();
        unsafe {
            std::env::remove_var("PORT");
        }
        assert_eq!(config.port, expected_port);
    }

    #[test]
    fn should_get_default_port_if_no_environment_variable() {
        let _guard = ENV_LOCK.lock().unwrap();
        unsafe {
            std::env::remove_var("PORT");
        }
        let config = Config::parse();
        assert_eq!(config.port, "50051");
    }
}
