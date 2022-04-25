use std::fs::File;
use std::io::BufReader;
use std::collections::VecDeque;

pub struct BinaryFile {
    internal_file: BufReader<File>,
    buffer: VecDeque<bool>,
}

impl BinaryFile {
    pub fn open_file(path: &str) -> std::io::Result<BinaryFile> {
        Ok(BinaryFile {
            internal_file: BufReader::new(File::open(path)?),
            buffer: VecDeque::new(),
        })
    }
}