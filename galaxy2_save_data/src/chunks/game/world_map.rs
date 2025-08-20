//! Types associated with World Map state.

use binrw::binrw;
use galaxy_save_core::{bin::Chunk, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for World Map state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SaveDataStorageWorldMap {
    /// The flags representing the Star Barrier passage status for each world.
    pub star_check_point_flag: [u8; Self::WORLD_CAPACITY],

    /// The positive world number currently being navigated.
    pub world_no: u8,
}

impl SaveDataStorageWorldMap {
    /// The number of worlds.
    const WORLD_NUM: usize = 7;

    /// The maximum number of worlds with data that can be stored.
    pub(super) const WORLD_CAPACITY: usize = Self::WORLD_NUM.next_power_of_two();
}

impl Default for SaveDataStorageWorldMap {
    fn default() -> Self {
        Self {
            star_check_point_flag: Default::default(),
            world_no: 1,
        }
    }
}

impl Chunk for SaveDataStorageWorldMap {
    fn hash_code() -> HashCode {
        let hash = HashCode::from("SaveDataStorageWorldMap")
            .into_raw()
            .wrapping_mul(9);

        HashCode::from_raw(hash)
    }
}
