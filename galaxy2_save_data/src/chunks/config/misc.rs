//! Types associated with miscellaneous user file state.

use binrw::binrw;
use galaxy_save_core::{bin::Chunk, hash::HashCode, time::Time};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for miscellaneous user file state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct ConfigDataMisc {
    /// The timestamp representing when the user file was most recently saved.
    pub last_modified: Time,
}

impl Chunk for ConfigDataMisc {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x1)
    }
}
