//! A contiguous space-efficient array of 8 bits.

use std::array;

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, SerializeTuple, Serializer},
};

#[cfg(test)]
mod tests;

/// A contiguous space-efficient array of 8 bits.
#[binrw]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[repr(transparent)]
pub struct BitArray8 {
    inner: u8,
}

impl BitArray8 {
    /// The number of bits the array can hold.
    pub const CAPACITY: usize = u8::BITS as usize;

    /// Creates a new `BitArray8`.
    ///
    /// # Examples
    ///
    /// ```
    /// use galaxy_save_core::array::BitArray8;
    ///
    /// let a = BitArray8::new();
    /// ```
    pub const fn new() -> Self {
        Self { inner: 0 }
    }

    /// Determines if the value of the given bit is equal to `1`.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether
    /// overflow checks are enabled.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use galaxy_save_core::array::BitArray8;
    ///
    /// let a = BitArray8::from(0b00001111);
    /// assert_eq!(a.test(0), true);
    /// assert_eq!(a.test(4), false);
    pub fn test(&self, index: u32) -> bool {
        let Some(mask) = 1u8.checked_shl(index) else {
            panic!(
                "index out of bounds: the len is {} but the index is {index}",
                Self::CAPACITY
            );
        };

        self.inner & mask != 0
    }

    /// Updates the value of the given bit to equal `1`.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether
    /// overflow checks are enabled.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use galaxy_save_core::array::BitArray8;
    ///
    /// let mut a = BitArray8::new();
    /// a.set(2);
    /// assert_eq!(a, BitArray8::from(0b00000100));
    /// ```
    pub fn set(&mut self, index: u32) {
        let Some(mask) = 1u8.checked_shl(index) else {
            panic!(
                "index out of bounds: the len is {} but the index is {index}",
                Self::CAPACITY
            );
        };

        self.inner |= mask;
    }

    /// Updates the value of the given bit to equal `0`.
    ///
    /// # Panics
    ///
    /// This function will always panic on overflow, regardless of whether
    /// overflow checks are enabled.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use galaxy_save_core::array::BitArray8;
    ///
    /// let mut a = BitArray8::from(0b11111111);
    /// a.clear(3);
    /// assert_eq!(a, BitArray8::from(0b11110111));
    /// ```
    pub fn clear(&mut self, index: u32) {
        let Some(mask) = 1u8.checked_shl(index) else {
            panic!(
                "index out of bounds: the len is {} but the index is {index}",
                Self::CAPACITY
            );
        };

        self.inner &= !mask;
    }
}

impl From<u8> for BitArray8 {
    fn from(value: u8) -> Self {
        Self { inner: value }
    }
}

impl From<[bool; BitArray8::CAPACITY]> for BitArray8 {
    fn from(array: [bool; Self::CAPACITY]) -> Self {
        let mut value = 0u8;

        for (index, b) in array.iter().copied().enumerate() {
            value |= (b as u8) << index;
        }

        Self { inner: value }
    }
}

impl From<BitArray8> for [bool; BitArray8::CAPACITY] {
    fn from(array: BitArray8) -> Self {
        array::from_fn(|i| array.inner & 1u8 << i != 0)
    }
}

#[cfg(feature = "serde")]
impl Serialize for BitArray8 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut tuple = serializer.serialize_tuple(Self::CAPACITY)?;

        for index in 0..Self::CAPACITY {
            let is_on = self.inner & 1u8 << index != 0;

            tuple.serialize_element(&is_on)?;
        }

        tuple.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for BitArray8 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let array = <[bool; Self::CAPACITY]>::deserialize(deserializer)?;

        Ok(Self::from(array))
    }
}
