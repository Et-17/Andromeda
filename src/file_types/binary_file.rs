use std::fs::File;
use std::io::Read;
use std::io::Bytes;
use std::collections::VecDeque;

pub struct BinaryFile {
    file_iterator: Bytes<File>,
    buffer: VecDeque<bool>,
}

impl BinaryFile {
    pub fn open_file(path: &str) -> std::io::Result<BinaryFile> {
        Ok(BinaryFile {
            file_iterator: File::open(path)?.bytes(),
            buffer: VecDeque::new(),
        })
    }

    pub fn load_byte(&mut self) -> Option<u8> {
        let next_byte = match self.file_iterator.next() {
            Some(byte) => byte.unwrap(),
            None => return None,
        };
        for i in (0..8).rev() {
            self.buffer.push_back((next_byte & (1 << i)) >= 1);
        };
        Some(next_byte)
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        match self.buffer.pop_front() {
            Some(bit) => Some(bit),
            None => {
                match self.load_byte() {
                    Some(_) => self.read_bit(),
                    None => None,
                }
            },
        }
    }
}