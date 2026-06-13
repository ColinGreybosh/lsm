use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
};

use crate::wal::message::{Key, Message, Value};

pub trait LogWriter {
    fn append(&mut self, key: &Key, value: &Value) -> Result<(), ()>;
}

#[derive(Debug)]
pub struct FileLogWriter {
    file: File,
}

impl FileLogWriter {
    pub fn new() -> Result<Self, Error> {
        match OpenOptions::new().append(true).create(true).open("wal") {
            Ok(file) => Ok(FileLogWriter { file }),
            Err(err) => Err(err),
        }
    }
}

impl LogWriter for FileLogWriter {
    fn append(&mut self, key: &Key, value: &Value) -> Result<(), ()> {
        let message = Message::set(key.clone(), value.clone());
        let mut serialized = message.to_bytes();
        serialized.push(0);
        match self.file.write(&serialized) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
