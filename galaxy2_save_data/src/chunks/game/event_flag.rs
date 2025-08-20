//! Types associated with key-value pair state, where each value is a Boolean.

use binrw::binrw;
use galaxy_save_core::{bin::Chunk, event::GameEventFlag, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for key-value pair state, where each value is a Boolean.
#[binrw]
#[br(import(data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SaveDataStorageEventFlag {
    /// The collection of key-value pairs.
    #[br(count = data_size / size_of::<GameEventFlag>())]
    event_flag: Vec<GameEventFlag>,
}

impl SaveDataStorageEventFlag {
    /// Returns the value corresponding to the key.
    pub fn get(&self, key: impl Into<HashCode>) -> Option<bool> {
        let key = key.into();

        self.event_flag
            .iter()
            .find(|f| **f == key)
            .map(|f| f.value())
    }

    /// Sets the value corresponding to the key.
    pub fn set(&mut self, key: impl Into<HashCode>, value: bool) {
        let key = key.into();

        if let Some(flag) = self.event_flag.iter_mut().find(|f| **f == key) {
            flag.set(value);
        }
    }
}

impl Chunk for SaveDataStorageEventFlag {
    fn hash_code() -> HashCode {
        HashCode::from("2bytes/flag")
    }
}
