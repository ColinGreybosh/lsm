use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Error},
    path::Path,
};

use crate::wal::message::Message;

pub trait LogReader {
    fn read(&self) -> Result<Vec<Message>, Error>;
}

#[derive(Debug)]
pub struct FileLogReader {
    file: File,
}

impl FileLogReader {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self, Error> {
        let file_path = base_path.as_ref().join("log.wal");
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;
        Ok(Self { file })
    }
}

impl LogReader for FileLogReader {
    fn read(&self) -> Result<Vec<Message>, Error> {
        let file_clone = match self.file.try_clone() {
            Ok(f) => f,
            Err(err) => return Err(err),
        };
        let reader = BufReader::new(file_clone);
        let mut messages = vec![];
        for chunk_result in reader.split(0) {
            let chunk = chunk_result.unwrap();
            if chunk.is_empty() {
                continue;
            }
            let message = Message::from_bytes(chunk);
            messages.push(message);
        }
        Ok(messages)
    }
}
