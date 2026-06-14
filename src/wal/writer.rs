use std::{
    fs::{File, OpenOptions},
    io::{Error, Write},
    path::Path,
};

use crate::wal::message::Message;

pub trait LogWriter {
    fn append(&mut self, message: &Message) -> Result<(), ()>;
}

#[derive(Debug)]
pub struct FileLogWriter {
    file: File,
}

impl FileLogWriter {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self, Error> {
        let file_path = base_path.as_ref().join("log.wal");
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        Ok(Self { file })
    }
}

impl LogWriter for FileLogWriter {
    fn append(&mut self, message: &Message) -> Result<(), ()> {
        let mut serialized = message.to_bytes();
        serialized.push(0);
        match self.file.write(&serialized) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
