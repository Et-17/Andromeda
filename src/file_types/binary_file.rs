// Andromeda by Et-17
//
// primary definition of BinaryFile reader

use crate::iters::Bits;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Bytes;
use std::io::Read;

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
        //Read the next byte on the file being read
        let next_byte = match self.file_iterator.next() {
            Some(byte) => byte.unwrap(),
            None => return None,
        };
        //Buffer the bits in the byte
        for i in (0..8).rev() {
            self.buffer.push_back((next_byte & (1 << i)) >= 1);
        }
        //Return the read byte for *flexibilty*
        Some(next_byte)
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        //Find the next buffered bit
        match self.buffer.pop_front() {
            Some(bit) => Some(bit),
            //If buffer is empty, buffer another byte
            None => match self.load_byte() {
                //Retry with new buffer or handle EOF
                Some(_) => self.read_bit(),
                None => None,
            },
        }
    }

    pub fn read_vec(&mut self, bits: i32) -> Option<Vec<bool>> {
        let mut read: Vec<bool> = Vec::new();
        //Read required bits
        for _ in 0..bits {
            //Store each read bit or handle EOF
            read.push(match self.read_bit() {
                Some(bit) => bit,
                None => return None,
            });
        }
        Some(read)
    }

    pub fn read_num(&mut self, bits: i32) -> Option<u32> {
        let mut read: u32 = 0;
        //Read required bits
        for _ in 0..bits {
            //Interpret the read bits as a binary string
            read *= 2;
            read += match self.read_bit() {
                Some(bit) => bit as u32,
                //Handle EOF
                None => return None,
            };
        }
        Some(read)
    }

    pub fn bits(self) -> Bits {
        Bits::new(self)
    }
}
