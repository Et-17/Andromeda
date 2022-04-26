// Andromeda by Et-17
//
// primary definition of Bits iterator

use crate::file_types::binary_file::BinaryFile;

pub struct Bits {
    iterated_file: BinaryFile,
}

impl Bits {
    pub fn new(bf: BinaryFile) -> Bits {
        Bits { iterated_file: bf }
    }
}

impl Iterator for Bits {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        //read_bit already returns an option
        self.iterated_file.read_bit()
    }
}
