// Andromeda by Et-17
//
// main library file

//These imports will increase convience by allowing direct importation
//of the structs by users of Andromeda
pub use file_types::BinaryFile;
pub use iters::Bits;

//More automatic imports
pub mod file_types;
pub mod iters;

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PATH: &str = "sample_files\\valid_path";
    const INVALID_PATH: &str = "sample_files\\invalid_path";

    #[test]
    fn binary_file_constructor_panic() {
        //Open a file that is known to be valid
        let v_bin_f = BinaryFile::open_file(VALID_PATH);
        assert!(v_bin_f.is_ok());
        //Open a file that is known to be invalid
        let inv_bin_f = BinaryFile::open_file(INVALID_PATH);
        assert!(inv_bin_f.is_err());
    }

    #[test]
    fn binary_file_bit_read() {
        let mut bin_f = BinaryFile::open_file(VALID_PATH).unwrap();
        let correct_bits = vec![
            false, true, true, true, false, true, false, false, false, true, true,
        ];
        //Ensure that the read bits match the correct bits
        for bit in correct_bits {
            assert_eq!(bin_f.read_bit().unwrap(), bit);
        }
    }

    #[test]
    fn binary_file_vec_read() {
        let mut bin_f = BinaryFile::open_file(VALID_PATH).unwrap();
        let correct_bits = vec![
            false, true, true, true, false, true, false, false, false, true, true,
        ];
        //Ensure that the read bits match the correct bits
        assert_eq!(bin_f.read_vec(11).unwrap(), correct_bits);
    }

    #[test]
    fn binary_file_num_read() {
        let mut bin_f = BinaryFile::open_file(VALID_PATH).unwrap();
        //Ensure that the read bits match the correct bits
        assert_eq!(bin_f.read_num(11).unwrap(), 0b01110100011);
    }

    #[test]
    fn bits_read() {
        let bin_f = BinaryFile::open_file(VALID_PATH).unwrap();
        let mut bit_iter = bin_f.bits();
        let correct_bits = vec![
            false, true, true, true, false, true, false, false, false, true, true,
        ];
        //Ensure that the read bits match the correct bits
        for bit in correct_bits {
            assert_eq!(bit_iter.next().unwrap(), bit);
        }
    }

    #[test]
    fn binary_file_signed_read() {
        let mut bin_f = BinaryFile::open_file(VALID_PATH).unwrap();
        //Move forward a bit to get a negative result
        bin_f.read_bit();
        //Ensure that the read bits match the correct bits
        assert_eq!(bin_f.read_num_signed(10).unwrap(), -93);
    }
}
