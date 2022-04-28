// Andromeda by Et-17
//
// primary definition of BinaryFile reader

use crate::iters::Bits;
use std::collections::VecDeque;
use std::fs::File;
use std::io::Bytes;
use std::io::Read;

/// A special file manager for bit-by-bit reading. It wraps an [`Bytes`]
/// iterator of the file it's reading, and uses a [`VecDeque`] to buffer
/// the bits from the read byte, allowing seamless bit based reading.
/// # Examples
/// ## Opening a file
/// You can open a file very simply
/// ```
/// # use andromeda::BinaryFile;
/// # let valid_path = "sample_files\\valid_path";
/// let bin_f = BinaryFile::open_file(valid_path);
/// assert!(bin_f.is_ok());
/// ```
/// However, if you try to open an invalid file, it will return an error
/// ```
/// # use andromeda::BinaryFile;
/// # let invalid_path = "sample_files\\invalid_path";
/// let inv_bin_f = BinaryFile::open_file(invalid_path);
/// assert!(inv_bin_f.is_err());
/// ```
/// ## Reading a file
/// Once the file is open, you can immediately start reading bits from
/// the newly opened file
/// ```
/// # use andromeda::BinaryFile;
/// # let valid_path = "sample_files\\valid_path";
/// // open the BinaryFile
/// let mut bin_f = BinaryFile::open_file(valid_path).unwrap();
/// // read a bit
/// let next_bit = bin_f.read_bit().unwrap();
/// ```
/// You can detect EOF just like you would detect an iterator running
/// out of things to read: `read_bit()` returns a `Box<bool>`,
/// which will contain [`None`] when it reaches EOF.
/// ```
/// # use andromeda::BinaryFile;
/// # let empty_file = "sample_files\\empty_file";
/// // open the BinaryFile
/// let mut bin_f = BinaryFile::open_file(empty_file).unwrap();
/// // read a bit, and verify eof
/// assert!(bin_f.read_bit().is_none());
/// ```
pub struct BinaryFile {
    ///An iterator over the bytes of the file being read
    file_iterator: Bytes<File>,
    ///The bufferred bits. Direct access of this is not recommended.
    pub buffer: VecDeque<bool>,
}

impl BinaryFile {
    ///Opens a file and creates a `BinaryFile` to read it
    pub fn open_file(path: &str) -> std::io::Result<BinaryFile> {
        Ok(BinaryFile {
            file_iterator: File::open(path)?.bytes(),
            buffer: VecDeque::new(),
        })
    }

    /// Buffers a new byte. Usually, you do not need to call this method
    /// directly as `read_bit()` automatically buffer another byte, but
    /// there are legitamet reasons for you to call this function
    /// directly, such as prebuffering bytes for a large read.
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

    ///Reads a bit from the file, automatically handling the buffering
    /// of new bytes
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

    /// Reads an aribtrary number of bits into a vector
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

    /// Reads an arbitrary number of bits into an unsigned integer
    pub fn read_num(&mut self, bits: u32) -> Option<u32> {
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

    /// Reads an arbitrary number of bits into a signed integer, using
    /// two's complement
    pub fn read_num_signed(&mut self, bits: u32) -> Option<i32> {
        let mut read: i32 = 0;
        let negative = match self.read_bit() {
            Some(bit) => bit,
            None => return None,
        };
        for _ in 0..(bits - 1) {
            read *= 2;
            if negative {
                read -= match self.read_bit() {
                    Some(bit) => !bit as i32,
                    None => return None,
                };
            } else {
                read += match self.read_bit() {
                    Some(bit) => bit as i32,
                    None => return None,
                }
            }
        }
        Some(read - negative as i32)
    }

    /// Creates a `Bits` iterator of the file
    pub fn bits(self) -> Bits {
        Bits::from(self)
    }
}
