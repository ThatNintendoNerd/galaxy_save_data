use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::hash::HashCode;

/// A key-value pair for a 16-bit unsigned integer.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct GameEventValue {
    /// The hashed key, truncated to the least significant 16 bits.
    key: u16,

    /// The associated value.
    value: u16,
}

impl GameEventValue {
    /// Creates a new `GameEventValue`.
    pub fn new(key: impl Into<HashCode>, value: u16) -> Self {
        let key = key.into().into_raw() as u16;

        Self { key, value }
    }

    /// Returns the hashed key.
    const fn key(&self) -> u16 {
        self.key
    }

    /// Returns a reference to the associated value.
    pub const fn value(&self) -> &u16 {
        &self.value
    }

    /// Returns a mutable reference to the associated value.
    pub const fn value_mut(&mut self) -> &mut u16 {
        &mut self.value
    }
}

impl PartialEq<HashCode> for GameEventValue {
    fn eq(&self, other: &HashCode) -> bool {
        self.key() == other.into_raw() as u16
    }
}
