use std::collections::HashMap;
use std::path::Path;

use crate::wal::log::{FileWal, WriteAheadLog};
use crate::wal::message::{Key, Message, Value};

pub mod cli;
pub mod wal;

pub trait Keyable {
    fn put(&mut self, key: Key, value: Value) -> std::io::Result<()>;
    fn get(&self, key: &Key) -> Option<&Value>;
    fn del(&mut self, key: Key) -> std::io::Result<()>;
    fn clear(&mut self) -> std::io::Result<()>;
}

#[derive(Debug)]
pub struct LogStructuredMergeTree {
    wal: FileWal,
    map: HashMap<Key, Value>,
}

impl Keyable for LogStructuredMergeTree {
    fn put(&mut self, key: Key, value: Value) -> std::io::Result<()> {
        let message = Message::Set {
            key: key.clone(),
            value: value.clone(),
        };
        self.wal.append(&message)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn get(&self, key: &Key) -> Option<&Value> {
        self.map.get(key)
    }

    fn del(&mut self, key: Key) -> std::io::Result<()> {
        let message = Message::Del { key: key.clone() };
        self.wal.append(&message)?;
        self.map.remove(&key);
        Ok(())
    }

    fn clear(&mut self) -> std::io::Result<()> {
        let message = Message::Clear {};
        self.wal.append(&message)?;
        self.map.clear();
        Ok(())
    }
}

impl LogStructuredMergeTree {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        std::fs::create_dir_all(&base_path).expect("Failed to create WAL directory");
        let mut map = HashMap::new();
        let mut wal = FileWal::new(&base_path).unwrap();
        if let Ok(messages) = wal.read_all() {
            for message in messages {
                match message {
                    Message::Set { key, value } => {
                        map.insert(key, value);
                    }
                    Message::Del { key } => {
                        map.remove(&key);
                    }
                    Message::Clear {} => {
                        map.clear();
                    }
                };
            }
        }
        Self { wal, map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn should_write_and_read_data() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert!(
            lsm.put(Key::from("my_key"), Value::from("some_value"))
                .is_ok()
        );
        std::assert_eq!(
            lsm.get(&Key::from("my_key")),
            Some(Value::from("some_value")).as_ref()
        );
    }

    #[test]
    fn should_return_none_for_nonexistent_key() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert_eq!(lsm.get(&Key::from("some_key")), None);
    }

    #[test]
    fn should_overwrite_value() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert!(
            lsm.put(Key::from("my_key"), Value::from("some_value"))
                .is_ok(),
        );
        std::assert_eq!(
            lsm.get(&Key::from("my_key")),
            Some(Value::from("some_value")).as_ref()
        );

        std::assert!(
            lsm.put(Key::from("my_key"), Value::from("some_new_value"))
                .is_ok()
        );
        std::assert_eq!(
            lsm.get(&Key::from("my_key")),
            Some(Value::from("some_new_value")).as_ref()
        );
    }

    #[test]
    fn should_delete_key() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert!(
            lsm.put(Key::from("my_key"), Value::from("some_value"))
                .is_ok(),
        );
        std::assert_eq!(
            lsm.get(&Key::from("my_key")),
            Some(Value::from("some_value")).as_ref()
        );

        std::assert!(lsm.del(Key::from("my_key")).is_ok());
        std::assert_eq!(lsm.get(&Key::from("my_key")), None);
    }

    #[test]
    fn should_clear_keys() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert!(
            lsm.put(Key::from("my_key_1"), Value::from("some_value_1"))
                .is_ok()
        );
        std::assert_eq!(
            lsm.get(&Key::from("my_key_1")),
            Some(Value::from("some_value_1")).as_ref()
        );

        std::assert!(
            lsm.put(Key::from("my_key_2"), Value::from("some_value_2"))
                .is_ok()
        );
        std::assert_eq!(
            lsm.get(&Key::from("my_key_2")),
            Some(Value::from("some_value_2")).as_ref()
        );

        std::assert!(lsm.clear().is_ok());
        std::assert_eq!(lsm.get(&Key::from("my_key_1")), None);
        std::assert_eq!(lsm.get(&Key::from("my_key_2")), None);
    }
}
