use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::hash::HashCode;

/// A key-value pair for a Boolean.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(from = "ExpandedGameEventFlag", into = "ExpandedGameEventFlag")
)]
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GameEventFlag {
    inner: u16,
}

impl GameEventFlag {
    /// The bitmask for the hashed key.
    const KEY_MASK: u16 = !Self::VALUE_MASK;

    /// The bitmask for the associated value.
    const VALUE_MASK: u16 = 1 << (u16::BITS - 1);

    /// The bit shift operand for the associated value.
    const VALUE_SHIFT: u32 = Self::VALUE_MASK.trailing_zeros();

    /// Creates a new `GameEventFlag`.
    pub fn new(key: impl Into<HashCode>, value: bool) -> Self {
        let key = key.into().into_raw() as u16 & Self::KEY_MASK;
        let value = (value as u16) << Self::VALUE_SHIFT;

        Self { inner: key | value }
    }

    /// Extracts the hashed key.
    const fn key(&self) -> u16 {
        self.inner & Self::KEY_MASK
    }

    /// Extracts the associated value.
    pub const fn value(&self) -> bool {
        self.inner & Self::VALUE_MASK != 0
    }

    /// Updates the associated value.
    pub const fn set(&mut self, value: bool) {
        self.inner = (self.inner & Self::KEY_MASK) | ((value as u16) << Self::VALUE_SHIFT);
    }
}

impl PartialEq<HashCode> for GameEventFlag {
    fn eq(&self, other: &HashCode) -> bool {
        self.key() == other.into_raw() as u16 & Self::KEY_MASK
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ExpandedGameEventFlag {
    key: u16,
    value: bool,
}

#[cfg(feature = "serde")]
impl From<GameEventFlag> for ExpandedGameEventFlag {
    fn from(flag: GameEventFlag) -> Self {
        Self {
            key: flag.key(),
            value: flag.value(),
        }
    }
}

#[cfg(feature = "serde")]
impl From<ExpandedGameEventFlag> for GameEventFlag {
    fn from(flag: ExpandedGameEventFlag) -> Self {
        let key = flag.key & Self::KEY_MASK;
        let value = (flag.value as u16) << Self::VALUE_SHIFT;

        Self { inner: key | value }
    }
}
