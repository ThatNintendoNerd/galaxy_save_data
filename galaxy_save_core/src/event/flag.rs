use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::hash::HashCode;

/// A key-value pair for a Boolean.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "ExpandedGameEventFlag", into = "ExpandedGameEventFlag")
)]
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct GameEventFlag {
    inner: u16,
}

impl GameEventFlag {
    /// The bitmask for the hashed key.
    const KEY_MASK: u16 = !Self::VALUE_MASK;

    /// The bit width of the hashed key.
    const KEY_WIDTH: u32 = u16::BITS - 1;

    /// The bitmask for the associated value.
    const VALUE_MASK: u16 = 1 << Self::KEY_WIDTH;

    /// The bit shift operand for the associated value.
    const VALUE_SHIFT: u32 = Self::KEY_WIDTH;

    /// Creates a new `GameEventFlag`.
    pub fn new(key: impl Into<HashCode>, value: bool) -> Self {
        let key = key.into().trunc() & Self::KEY_MASK;
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
        self.inner = self.key() | ((value as u16) << Self::VALUE_SHIFT);
    }
}

impl PartialEq<HashCode> for GameEventFlag {
    fn eq(&self, other: &HashCode) -> bool {
        self.key() == other.trunc() & Self::KEY_MASK
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ExpandedGameEventFlag {
    key: String,
    value: bool,
}

#[cfg(feature = "serde")]
impl From<GameEventFlag> for ExpandedGameEventFlag {
    fn from(flag: GameEventFlag) -> Self {
        let hash = HashCode::from_raw(flag.key().into());
        let label = hash.to_label(Some(GameEventFlag::KEY_WIDTH));

        Self {
            key: label,
            value: flag.value(),
        }
    }
}

#[cfg(feature = "serde")]
impl TryFrom<ExpandedGameEventFlag> for GameEventFlag {
    type Error = crate::hash::FromLabelError;

    fn try_from(flag: ExpandedGameEventFlag) -> Result<Self, Self::Error> {
        let hash = HashCode::from_label(&flag.key)?;

        Ok(Self::new(hash, flag.value))
    }
}
