//! Basic memory utilities.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// The wrapper type for the result of the checksum function.
///
/// # Algorithm
///
/// Given an arbitrary data buffer, the checksum algorithm processes the data
/// as a series of unsigned 16-bit integers. Consequently, if the size of the
/// data buffer does not evenly divide by that of the chunk size, the remainder
/// will be omitted from the computation.
///
/// These integers are iterated over and added to two variables, the first of
/// which tracks the sum of all the values and the other tracks the sum of all
/// the values' bitwise complement. After iteration, the two variables are
/// concatenated into an unsigned 32-bit integer, where the sum of all the
/// values is shifted to occupy the most significant 16 bits.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Checksum {
    inner: u32,
}

impl Checksum {
    /// Computes a new `Checksum` from a byte slice.
    fn from_bytes<F>(buf: &[u8], f: F) -> Self
    where
        F: Fn([u8; 2]) -> u16,
    {
        let mut sum: u16 = 0;
        let mut inv_sum: u16 = 0;

        for term in buf.chunks_exact(size_of::<u16>()).map(|c| f([c[0], c[1]])) {
            sum = sum.wrapping_add(term);
            inv_sum = inv_sum.wrapping_add(!term);
        }

        Self {
            inner: ((sum as u32) << u16::BITS) | inv_sum as u32,
        }
    }

    /// Computes a new `Checksum` from a byte slice in big endian.
    pub fn from_be_bytes(buf: &[u8]) -> Self {
        Self::from_bytes(buf, u16::from_be_bytes)
    }

    /// Computes a new `Checksum` from a byte slice in little endian.
    pub fn from_le_bytes(buf: &[u8]) -> Self {
        Self::from_bytes(buf, u16::from_le_bytes)
    }

    /// Creates a new `Checksum` from a raw checksum.
    pub const fn from_raw(checksum: u32) -> Self {
        Self { inner: checksum }
    }

    /// Consumes the `Checksum`, returning the contained value.
    pub const fn into_raw(self) -> u32 {
        self.inner
    }
}
