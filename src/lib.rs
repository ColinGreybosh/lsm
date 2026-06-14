use std::collections::HashMap;
use std::path::Path;

use crate::wal::message::{Key, Message, Value};
use crate::wal::reader::{FileLogReader, LogReader};
use crate::wal::writer::{FileLogWriter, LogWriter};

pub mod cli;
pub mod wal;

pub trait Keyable {
    fn put(&mut self, key: Key, value: Value) -> Result<(), ()>;
    fn get(&self, key: Key) -> Option<Value>;
    fn del(&mut self, key: Key) -> Result<(), ()>;
    fn clear(&mut self) -> Result<(), ()>;
}

#[derive(Debug)]
pub struct LogStructuredMergeTree<Log = FileLogWriter>
where
    Log: LogWriter,
{
    wal: Log,
    map: HashMap<Key, Value>,
}

impl<Log: LogWriter> Keyable for LogStructuredMergeTree<Log> {
    fn put(&mut self, key: Key, value: Value) -> Result<(), ()> {
        let message = Message::Set {
            key: key.clone(),
            value: value.clone(),
        };
        self.wal.append(&message)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn get(&self, key: Key) -> Option<Value> {
        match self.map.get(&key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }

    fn del(&mut self, key: Key) -> Result<(), ()> {
        let message = Message::Del { key: key.clone() };
        self.wal.append(&message)?;
        self.map.remove(&key);
        Ok(())
    }

    fn clear(&mut self) -> Result<(), ()> {
        let message = Message::Clear {};
        self.wal.append(&message)?;
        self.map.clear();
        Ok(())
    }
}

impl LogStructuredMergeTree<FileLogWriter> {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        std::fs::create_dir_all(&base_path).expect("Failed to create WAL directory");
        let mut map = HashMap::new();
        let reader = FileLogReader::new(&base_path).unwrap();
        let messages = reader.read();
        match messages {
            Ok(messages) => {
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
            Err(_) => panic!("Failed to initialize in-memory store"),
        }
        let wal = FileLogWriter::new(&base_path).unwrap();
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

        std::assert_eq!(
            lsm.put(Key::from("my_key"), Value::from("some_value")),
            Ok(())
        );
        std::assert_eq!(
            lsm.get(Key::from("my_key")),
            Some(Value::from("some_value"))
        );
    }

    #[test]
    fn should_return_none_for_nonexistent_key() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert_eq!(lsm.get(Key::from("some_key")), None);
    }

    #[test]
    fn should_overwrite_value() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert_eq!(
            lsm.put(Key::from("my_key"), Value::from("some_value")),
            Ok(())
        );
        std::assert_eq!(
            lsm.get(Key::from("my_key")),
            Some(Value::from("some_value"))
        );

        std::assert_eq!(
            lsm.put(Key::from("my_key"), Value::from("some_new_value")),
            Ok(())
        );
        std::assert_eq!(
            lsm.get(Key::from("my_key")),
            Some(Value::from("some_new_value"))
        );
    }

    #[test]
    fn should_delete_key() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert_eq!(
            lsm.put(Key::from("my_key"), Value::from("some_value")),
            Ok(())
        );
        std::assert_eq!(
            lsm.get(Key::from("my_key")),
            Some(Value::from("some_value"))
        );

        std::assert_eq!(lsm.del(Key::from("my_key")), Ok(()));
        std::assert_eq!(lsm.get(Key::from("my_key")), None);
    }

    #[test]
    fn should_clear_keys() {
        let tmp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut lsm = LogStructuredMergeTree::new(tmp_dir.path());

        std::assert_eq!(
            lsm.put(Key::from("my_key_1"), Value::from("some_value_1")),
            Ok(())
        );
        std::assert_eq!(
            lsm.get(Key::from("my_key_1")),
            Some(Value::from("some_value_1"))
        );

        std::assert_eq!(
            lsm.put(Key::from("my_key_2"), Value::from("some_value_2")),
            Ok(())
        );
        std::assert_eq!(
            lsm.get(Key::from("my_key_2")),
            Some(Value::from("some_value_2"))
        );

        std::assert_eq!(lsm.clear(), Ok(()));
        std::assert_eq!(lsm.get(Key::from("my_key_1")), None);
        std::assert_eq!(lsm.get(Key::from("my_key_2")), None);
    }
}
