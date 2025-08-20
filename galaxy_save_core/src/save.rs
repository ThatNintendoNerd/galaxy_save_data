//! Basic save file format utilities.

use std::io;

use thiserror::Error;

use crate::mem::Checksum;

/// A trait for standardizing save file header implementation.
pub trait SaveFileHeader {
    /// The expected version number of the save data.
    const VERSION: u32;

    /// The maximum allowed number of user file descriptors.
    const USER_FILE_INFO_MAX: u32;

    /// The maximum allowed size of the file, in bytes.
    const FILE_SIZE_MAX: u32;

    /// Returns the precomputed checksum derived from the file data.
    fn checksum(&self) -> Checksum;

    /// Returns the version number of the save data.
    fn version(&self) -> u32;

    /// Returns the number of stored user file descriptors.
    fn user_file_info_num(&self) -> u32;

    /// Returns the size of the file, in bytes.
    fn file_size(&self) -> u32;

    /// Validates the save file header in big-endian.
    fn check_be(&self, buf: &[u8]) -> Result<(), CheckSaveFileError> {
        self.check(buf)?;

        if self.checksum() != Checksum::from_be_bytes(&buf[size_of::<Checksum>()..]) {
            return Err(CheckSaveFileError::InequalChecksum);
        }

        Ok(())
    }

    /// Validates the save file header in little-endian.
    fn check_le(&self, buf: &[u8]) -> Result<(), CheckSaveFileError> {
        self.check(buf)?;

        if self.checksum() != Checksum::from_le_bytes(&buf[size_of::<Checksum>()..]) {
            return Err(CheckSaveFileError::InequalChecksum);
        }

        Ok(())
    }

    /// Validates the save file header without validating the precomputed checksum.
    fn check(&self, buf: &[u8]) -> Result<(), CheckSaveFileError> {
        if self.version() != Self::VERSION {
            return Err(CheckSaveFileError::IncompatibleVersion {
                expected: Self::VERSION,
                found: self.version(),
            });
        }

        if self.file_size() as usize != buf.len() {
            return Err(CheckSaveFileError::InequalFileSize {
                expected: self.file_size(),
                found: buf.len(),
            });
        }

        if self.file_size() > Self::FILE_SIZE_MAX {
            return Err(CheckSaveFileError::InvalidFileSize {
                expected_max: Self::FILE_SIZE_MAX,
                found: self.file_size(),
            });
        }

        if self.user_file_info_num() > Self::USER_FILE_INFO_MAX {
            return Err(CheckSaveFileError::InvalidUserFileInfoNum {
                expected_max: Self::USER_FILE_INFO_MAX,
                found: self.user_file_info_num(),
            });
        }

        Ok(())
    }
}

/// An error returned from validating the save file.
#[derive(Debug, Error)]
pub enum CheckSaveFileError {
    /// An error occurred while reading the file.
    #[error(transparent)]
    Io(#[from] io::Error),

    /// An error occurred while parsing the file.
    #[error(transparent)]
    Binrw(#[from] binrw::Error),

    /// The parsed version number does not equal what was expected.
    #[error("expected version number {expected}, found version number {found}")]
    IncompatibleVersion {
        /// The expected version number.
        expected: u32,

        /// The parsed version number.
        found: u32,
    },

    /// The parsed size of the file does not equal the actual size.
    #[error("expected a file size of {expected} bytes, found {found} bytes")]
    InequalFileSize {
        /// The parsed size of the file.
        expected: u32,

        /// The actual size of the file.
        found: usize,
    },

    /// The parsed size of the file exceeds the buffer's capacity.
    #[error("expected a maximum file size of {expected_max} bytes, found {found} bytes")]
    InvalidFileSize {
        /// The expected maximum size of the file.
        expected_max: u32,

        /// The parsed size of the file.
        found: u32,
    },

    /// The parsed number of user file descriptors exceeds what was expected.
    #[error(
        "expected a maximum number of {expected_max} user file descriptors, found {found} user file descriptors"
    )]
    InvalidUserFileInfoNum {
        /// The expected maximum number of user file descriptors.
        expected_max: u32,

        /// The parsed number of user file descriptors.
        found: u32,
    },

    /// The precomputed checksum does not equal the newly computed checksum.
    #[error("the precomputed checksum does not equal the newly computed checksum")]
    InequalChecksum,
}
