pub use crate::file_types::binary_file::BinaryFile;

pub mod file_types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_file_constructor_panic() {
        let v_bin_f = BinaryFile::open_file("sample_files\\valid_path");
        assert!(v_bin_f.is_ok());
        let inv_bin_f = BinaryFile::open_file("sample_files\\invalid_path");
        assert!(inv_bin_f.is_err());
    }

    #[test]
    fn binary_file_bit_read() {
        let mut bin_f = BinaryFile::open_file("sample_files\\valid_path").unwrap();
        let correct_bits = vec![false, true, true, true, false, true,
            false, false, false, true, true];
        for bit in correct_bits {
            assert_eq!(bin_f.read_bit().unwrap(), bit);
        }
    }

    #[test]
    fn binary_file_vec_read() {
        let mut bin_f = BinaryFile::open_file("sample_files\\valid_path").unwrap();
        let correct_bits = vec![false, true, true, true, false, true,
            false, false, false, true, true];
        assert_eq!(bin_f.read_vec(11).unwrap(), correct_bits);
    }

    #[test]
    fn binary_file_num_read() {
        let mut bin_f = BinaryFile::open_file("sample_files\\valid_path").unwrap();
        assert_eq!(bin_f.read_num(11).unwrap(), 0b01110100011);
    }
}
