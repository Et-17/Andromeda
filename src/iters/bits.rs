// Andromeda by Et-17
//
// primary definition of Bits iterator

use crate::file_types::binary_file::BinaryFile;

///A bit based file iterator
pub struct Bits {
    ///The internal file being iterated
    iterated_file: BinaryFile,
}

impl Bits {
    ///Creates a new Bits iterator from a BinaryFile
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
