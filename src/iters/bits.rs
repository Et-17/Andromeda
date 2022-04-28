// Andromeda by Et-17
//
// primary definition of Bits iterator

use crate::file_types::binary_file::BinaryFile;

/// An iterator that reads files bit-by-bit.
/// This is intended to aid in idiomatic usage of [`BinaryFile`], and is
/// essentially just the same as calling `read_bit()` on the
/// [`BinaryFile`] that it is scanning.
/// # Examples
/// In order to create a [`Bits`] iterator from a [`BinaryFile`], it's
/// as easy as doing the following
/// ```
/// # use andromeda::{Bits, BinaryFile};
/// # use std::io;
/// # let valid_path = "sample_files\\valid_path";
/// let mut bin_f = BinaryFile::open_file(valid_path)?;
/// let mut bits = Bits::from(bin_f);
/// # Ok::<(), io::Error>(())
/// ```
pub struct Bits {
    ///The internal file being iterated
    iterated_file: BinaryFile,
}

impl Bits {
    /// Creates a new [`Bits`] iterator from a [`BinaryFile`]. It
    /// continues reading from the byte that it the [`BinaryFile`] is
    /// on, meaning that when you call this method, it picks up in the
    /// same place as it would if you called it after clearning the
    /// internal buffer of the [`BinaryFile`]. See structure examples
    /// for more information on how to use this.
    pub fn from(bf: BinaryFile) -> Bits {
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
