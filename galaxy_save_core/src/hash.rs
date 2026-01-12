//! Basic hash utilities.

use std::num::ParseIntError;

use binrw::binrw;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod map;

#[cfg(test)]
mod tests;

pub use map::{HashCodeMap, ParseLabelError};

/// The wrapper type for the result of the hash function.
///
/// # Algorithm
///
/// Given an arbitrary data buffer, the hash algorithm processes the data as a
/// series of signed bytes converted to unsigned 32-bit integers. Initialized
/// to zero, the new value of the result variable will equal the sum of the
/// converted byte and the current value of the result variable, where the
/// latter is multiplied by the arbitrary constant `31`.
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
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct HashCode {
    inner: u32,
}

impl HashCode {
    /// The arbitrary prime number for hash dispersion.
    const PRIME: u32 = 31;

    /// Creates a new `HashCode` from a raw hash.
    pub const fn from_raw(hash: u32) -> Self {
        Self { inner: hash }
    }

    /// Consumes the `HashCode`, returning the contained value.
    pub const fn into_raw(self) -> u32 {
        self.inner
    }

    /// Returns the least significant 16 bits of `self`.
    pub const fn trunc(self) -> u16 {
        self.inner as u16
    }

    /// Converts a hexadecimal string into a `HashCode`.
    pub fn from_hex_str(s: &str) -> Result<Self, ParseHexError> {
        if let Some(stripped) = s.strip_prefix("0x") {
            Ok(HashCode::from_raw(u32::from_str_radix(stripped, 16)?))
        } else {
            Err(ParseHexError::MissingPrefix)
        }
    }

    /// Converts a label or hexadecimal string into a `HashCode`.
    ///
    /// To convert a string into a `HashCode` with only necessary operations,
    /// use `HashCode::from` instead.
    pub fn from_label(label: &str) -> Result<Self, FromLabelError> {
        match Self::from_hex_str(label) {
            Ok(hash) => Ok(hash),
            Err(error) => match error {
                ParseHexError::MissingPrefix => HashCodeMap::get()
                    .lock()
                    .hash_of(label)
                    .ok_or_else(|| FromLabelError::NotFound(label.into())),
                ParseHexError::ParseInt(_) => Err(error.into()),
            },
        }
    }

    /// Converts a `HashCode` back to its original label, or hexadecimal if not found.
    pub fn to_label(self, width: Option<u32>) -> String {
        HashCodeMap::get()
            .lock()
            .label_of(self, width)
            .cloned()
            .unwrap_or_else(|| format!("{:#X}", self.inner))
    }
}

impl From<&[u8]> for HashCode {
    fn from(buf: &[u8]) -> Self {
        let mut hash = 0u32;

        // Each byte is first casted to a signed byte because a value like
        // 0x80u8 should convert to 0xFFFFFF80u32 instead of 0x80u32.
        for byte in buf.iter().map(|b| b.cast_signed() as u32) {
            hash = byte.wrapping_add(hash.wrapping_mul(Self::PRIME));
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

impl From<HashCode16> for HashCode {
    fn from(hash: HashCode16) -> Self {
        Self {
            inner: hash.into_raw().into(),
        }
    }
}

impl PartialEq<HashCode16> for HashCode {
    fn eq(&self, other: &HashCode16) -> bool {
        self.trunc() == other.into_raw()
    }
}

#[cfg(feature = "serde")]
impl Serialize for HashCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let label = self.to_label(None);

        serializer.serialize_str(&label)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for HashCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let label = String::deserialize(deserializer)?;

        HashCode::from_label(&label).map_err(serde::de::Error::custom)
    }
}

/// The wrapper type for the result of the hash function, truncated to the
/// least significant 16 bits.
#[binrw]
#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct HashCode16 {
    inner: u16,
}

impl HashCode16 {
    /// Creates a new `HashCode16` from a raw hash.
    pub const fn from_raw(hash: u16) -> Self {
        Self { inner: hash }
    }

    /// Consumes the `HashCode16`, returning the contained value.
    pub const fn into_raw(self) -> u16 {
        self.inner
    }

    /// Converts a hexadecimal string into a `HashCode16`.
    pub fn from_hex_str(s: &str) -> Result<Self, ParseHexError> {
        HashCode::from_hex_str(s).map(|h| h.into())
    }

    /// Converts a label or hexadecimal string into a `HashCode16`.
    pub fn from_label(label: &str) -> Result<Self, FromLabelError> {
        HashCode::from_label(label).map(|h| h.into())
    }

    /// Converts a `HashCode16` back to its original label, or hexadecimal if not found.
    pub fn to_label(self) -> String {
        HashCode::from(self).to_label(Some(u16::BITS))
    }
}

impl From<&[u8]> for HashCode16 {
    fn from(buf: &[u8]) -> Self {
        HashCode::from(buf).into()
    }
}

impl From<&String> for HashCode16 {
    fn from(s: &String) -> Self {
        HashCode::from(s).into()
    }
}

impl From<&str> for HashCode16 {
    fn from(s: &str) -> Self {
        HashCode::from(s).into()
    }
}

impl From<HashCode> for HashCode16 {
    fn from(hash: HashCode) -> Self {
        Self {
            inner: hash.trunc(),
        }
    }
}

impl PartialEq<HashCode> for HashCode16 {
    fn eq(&self, other: &HashCode) -> bool {
        self.inner == other.trunc()
    }
}

#[cfg(feature = "serde")]
impl Serialize for HashCode16 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let label = self.to_label();

        serializer.serialize_str(&label)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for HashCode16 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hash = HashCode::deserialize(deserializer)?;

        Ok(hash.into())
    }
}

/// An error returned from converting a hexadecimal string into a [`HashCode`].
#[derive(Debug, Error)]
pub enum ParseHexError {
    /// An error occurred while parsing the prefix.
    #[error("the raw hash is missing the hexadecimal prefix")]
    MissingPrefix,

    /// An error occurred while parsing the integer portion.
    #[error(transparent)]
    ParseInt(#[from] ParseIntError),
}

/// An error returned from converting a string into a [`HashCode`].
#[derive(Debug, Error)]
pub enum FromLabelError {
    /// An error occurred while converting a hexadecimal string.
    #[error(transparent)]
    ParseHex(#[from] ParseHexError),

    /// An error occurred while searching for a label in the map.
    #[error("the label was not found in the map")]
    NotFound(String),
}
