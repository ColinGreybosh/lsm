use std::collections::HashMap;

pub trait Keyable {
    fn put(&mut self, key: &str, value: &str) -> Result<(),()>;
    fn get(&self, key: &str) -> Option<&str>;
}

struct LogStructuredMergeTree {
    map: HashMap<String, String>,
}

impl Keyable for LogStructuredMergeTree {
    fn put(&mut self, key: &str, value: &str) -> Result<(),()> {
        self.map.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn get(&self, key: &str) -> Option<&str> {
        match self.map.get(key) {
            Some(value) => Some(value),
            None => None,
        }
    }
}

impl LogStructuredMergeTree {
    pub fn new() -> LogStructuredMergeTree {
        let map = HashMap::new();
        LogStructuredMergeTree { map }    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_and_read_data() {
        let mut lsm = LogStructuredMergeTree::new();
        let write_result = lsm.put("my_key", "some_value");
        std::assert_matches!(write_result, Ok(()));
        let read_result = lsm.get("my_key");
        std::assert_matches!(read_result, Some("some_value"))
    }

    #[test]
    fn should_return_none_for_nonexistent_key() {
        let lsm = LogStructuredMergeTree::new();
        let read_result = lsm.get("some_key");
        std::assert_matches!(read_result, None);
    }

    #[test]
    fn should_overwrite_value() {
        let mut lsm = LogStructuredMergeTree::new();
        lsm.put("my_key", "some_value").unwrap();
        let first_read_result = lsm.get("my_key");
        std::assert_matches!(first_read_result, Some("some_value"));
        lsm.put("my_key", "some_new_value").unwrap();
        let second_read_result = lsm.get("my_key");
        std::assert_matches!(second_read_result, Some("some_new_value"));
    }
}
