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
}
