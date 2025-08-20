//! Types associated with key-value pair state, where each value is a 16-bit
//! unsigned integer.

use binrw::binrw;
use galaxy_save_core::{bin::Chunk, event::GameEventValue, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for key-value pair state, where each value is a 16-bit unsigned integer.
#[binrw]
#[br(import(data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SaveDataStorageEventValue {
    /// The collection of key-value pairs.
    #[br(count = data_size / size_of::<GameEventValue>())]
    event_value: Vec<GameEventValue>,
}

impl SaveDataStorageEventValue {
    /// Returns a reference to the value corresponding to the key.
    pub fn get(&self, key: impl Into<HashCode>) -> Option<&u16> {
        let key = key.into();

        self.event_value
            .iter()
            .find(|v| **v == key)
            .map(|v| v.value())
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut(&mut self, key: impl Into<HashCode>) -> Option<&mut u16> {
        let key = key.into();

        self.event_value
            .iter_mut()
            .find(|v| **v == key)
            .map(|v| v.value_mut())
    }
}

impl Chunk for SaveDataStorageEventValue {
    fn hash_code() -> HashCode {
        let hash = u32::from_be_bytes(*b"VLE1");

        HashCode::from_raw(hash)
    }
}
