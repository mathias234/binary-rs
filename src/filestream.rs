use crate::{Stream, StreamError};
use std::fs;
use std::io::prelude::*;
use std::io::SeekFrom;

pub enum OpenType {
    OpenAndCreate,
    Open,
}

pub struct Filestream {
    file: fs::File,
}

impl Filestream {
    pub fn new(filepath: &str, open_type: OpenType) -> Result<Filestream, StreamError> {
        let file;

        match open_type {
            OpenType::OpenAndCreate => file = fs::File::create(filepath),
            OpenType::Open => file = fs::File::open(filepath),
        }

        match file {
            Ok(f) => Ok(Filestream { file: f }),
            Err(_) => Err(StreamError::OpenError),
        }
    }
}

impl Stream for Filestream {
    fn write(&mut self, bytes: &Vec<u8>) -> Result<usize, StreamError> {
        match self.file.write(bytes) {
            Ok(res) => Ok(res),
            Err(_) => Err(StreamError::WriteError),
        }
    }

    fn read(&mut self, buffer: &mut Vec<u8>) -> Result<usize, StreamError> {
        if self.tell().unwrap() + buffer.len() > self.file.metadata().unwrap().len() as usize {
            return Err(StreamError::ReadError);
        }

        match self.file.read(buffer) {
            Ok(res) => Ok(res),
            Err(_) => Err(StreamError::ReadError),
        }
    }

    fn seek(&mut self, to: usize) -> Result<usize, StreamError> {
        match self.file.seek(SeekFrom::Start(to as u64)) {
            Ok(res) => Ok(res as usize),
            Err(_) => Err(StreamError::SeekError),
        }
    }

    fn tell(&mut self) -> Result<usize, StreamError> {
        match self.file.seek(SeekFrom::Current(0)) {
            Ok(res) => Ok(res as usize),
            Err(_) => Err(StreamError::TellError),
        }
    }
}
