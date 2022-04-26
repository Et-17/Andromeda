use std::fs::File;
use std::io::Read;
use std::io::Bytes;
use crate::iters::Bits;
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

    pub fn read_vec(&mut self, bits: i32) -> Option<Vec<bool>> {
        let mut read: Vec<bool> = Vec::new();
        for _ in 0..bits {
            read.push(match self.read_bit() {
                Some(bit) => bit,
                None => return None,
            });
        }
        Some(read)
    }

    pub fn read_num(&mut self, bits: i32) -> Option<u32> {
        let mut read: u32 = 0;
        for _ in 0..bits {
            read *= 2;
            read += match self.read_bit() {
                Some(bit) => bit as u32,
                None => return None,
            };
        }
        Some(read)
    }

    pub fn bits(self) -> Bits {
        Bits::new(self)
    }
}