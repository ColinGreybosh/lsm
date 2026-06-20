use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::Path,
};

use crate::wal::message::Message;

pub trait WriteAheadLog {
    fn read_all(&mut self) -> std::io::Result<Vec<Message>>;

    fn append(&mut self, message: &Message) -> std::io::Result<()>;

    fn rotate(&mut self) -> std::io::Result<()>;
}

#[derive(Debug)]
pub struct FileWal {
    file: File,
}

impl FileWal {
    pub fn new<P: AsRef<Path>>(base_path: P) -> std::io::Result<Self> {
        let file_path = base_path.as_ref().join("log.wal");
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&file_path)?;
        Ok(Self { file })
    }
}

impl WriteAheadLog for FileWal {
    fn read_all(&mut self) -> std::io::Result<Vec<Message>> {
        let file_clone = self.file.try_clone()?;
        let mut reader = BufReader::new(file_clone);

        reader.seek(std::io::SeekFrom::Start(0))?;

        let mut messages = vec![];
        for chunk_result in reader.split(0) {
            let chunk = chunk_result?;
            if chunk.is_empty() {
                continue;
            }
            messages.push(Message::from_bytes(&chunk));
        }
        Ok(messages)
    }

    fn append(&mut self, message: &Message) -> std::io::Result<()> {
        let mut serialized = message.to_bytes();
        serialized.push(0);
        self.file.write_all(&serialized)?;
        Ok(())
    }

    fn rotate(&mut self) -> std::io::Result<()> {
        self.file.set_len(0)?;
        self.file.seek(std::io::SeekFrom::Start(0))?;
        Ok(())
    }
}
