//! Basic hash utilities.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The wrapper type for the result of the hash function.
///
/// # Algorithm
///
/// Given an arbitrary data buffer, the hash algorithm processes the data as a
/// series of signed bytes converted to 32-bit integers. Initialized to zero,
/// the new value of the result variable will equal the sum of the converted
/// byte and the current value of the result variable, where the latter is
/// multiplied by the arbitrary constant `31`.
///
/// ## Differences from the C++ Implementation
///
/// The `MR::getHashCode` function takes a pointer to a constant signed byte
/// buffer. It will iterate over all bytes behind this pointer until it finds
/// one equal to zero, suggesting iteration over the characters of a
/// nul-terminated string.
///
/// In the Rust implementation, all bytes within a string slice will be
/// iterated over due to strings in Rust not using nul termination.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct HashCode {
    inner: u32,
}

impl HashCode {
    /// The arbitrary prime number for hash dispersion.
    const HASH_KEY: u32 = 31;

    /// Creates a new `HashCode` from a raw hash.
    pub const fn from_raw(hash: u32) -> Self {
        Self { inner: hash }
    }

    /// Consumes the `HashCode`, returning the contained value.
    pub const fn into_raw(self) -> u32 {
        self.inner
    }
}

impl From<&[u8]> for HashCode {
    fn from(buf: &[u8]) -> Self {
        let mut hash: u32 = 0;

        // Each byte is first casted to a signed byte because a value like
        // 0x80u8 should convert to 0xFFFFFF80u32 instead of 0x80u32.
        for byte in buf.iter().map(|b| b.cast_signed() as u32) {
            hash = byte.wrapping_add(hash.wrapping_mul(Self::HASH_KEY));
        }

        Self { inner: hash }
    }
}

impl From<&String> for HashCode {
    fn from(s: &String) -> Self {
        Self::from(s.as_bytes())
    }
}

impl From<&str> for HashCode {
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes())
    }
}
