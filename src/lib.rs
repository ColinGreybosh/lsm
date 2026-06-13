use std::collections::HashMap;

use crate::wal::message::{Key, Message, Value};
use crate::wal::reader::{FileLogReader, LogReader};
use crate::wal::writer::{FileLogWriter, LogWriter};

pub mod cli;
pub mod wal;

pub trait Keyable {
    fn put(&mut self, key: Key, value: Value) -> Result<(), ()>;
    fn get(&self, key: Key) -> Option<Value>;
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
        self.wal.append(&key, &value)?;
        self.map.insert(key, value);
        Ok(())
    }

    fn get(&self, key: Key) -> Option<Value> {
        match self.map.get(&key) {
            Some(value) => Some(value.clone()),
            None => None,
        }
    }
}

impl LogStructuredMergeTree<FileLogWriter> {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        let reader = FileLogReader::new().unwrap();
        let messages = reader.read();
        match messages {
            Ok(messages) => {
                for message in messages {
                    match message {
                        Message::Set { key, value } => map.insert(key, value),
                    };
                }
            }
            Err(_) => panic!("Failed to initialize in-memory store"),
        }
        let wal = FileLogWriter::new().unwrap();
        LogStructuredMergeTree { wal, map }
    }
}

impl<Log: LogWriter> LogStructuredMergeTree<Log> {
    pub fn with_wal(wal: Log) -> Self {
        let map = HashMap::new();
        LogStructuredMergeTree { wal, map }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_write_and_read_data() {
        let mut lsm = LogStructuredMergeTree::new();
        let write_result = lsm.put(Key::from("my_key"), Value::from("some_value"));
        std::assert_matches!(write_result, Ok(()));
        let read_result = lsm.get(Key::from("my_key"));
        std::assert!(match read_result {
            Some(value) => value == Value::from("some_value"),
            None => false,
        });
    }

    #[test]
    fn should_return_none_for_nonexistent_key() {
        let lsm = LogStructuredMergeTree::new();
        let read_result = lsm.get(Key::from("some_key"));
        std::assert_matches!(read_result, None);
    }

    #[test]
    fn should_overwrite_value() {
        let mut lsm = LogStructuredMergeTree::new();
        lsm.put(Key::from("my_key"), Value::from("some_value"))
            .unwrap();
        let first_read_result = lsm.get(Key::from("my_key"));
        std::assert!(match first_read_result {
            Some(value) => value == Value::from("some_value"),
            None => false,
        });
        lsm.put(Key::from("my_key"), Value::from("some_new_value"))
            .unwrap();
        let second_read_result = lsm.get(Key::from("my_key"));
        std::assert!(match second_read_result {
            Some(value) => value == Value::from("some_new_value"),
            None => false,
        });
    }
}
