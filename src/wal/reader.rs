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
    pub fn new() -> Result<Self, Error> {
        let file_path = Path::new("wal");
        if !file_path.exists() {
            match OpenOptions::new().create(true).write(true).open(file_path) {
                Ok(_) => (),
                Err(err) => return Err(err),
            }
        }
        match OpenOptions::new().read(true).open(file_path) {
            Ok(file) => Ok(FileLogReader { file }),
            Err(err) => Err(err),
        }
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
