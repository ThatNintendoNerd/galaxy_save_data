//! Types associated with user file creation state.

use binrw::binrw;
use galaxy_save_core::{bin::Chunk, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for user file creation state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct ConfigDataCreate {
    /// Determines if the user file currently exists.
    #[br(map = |b: i8| b != 0)]
    #[bw(map = |b| -i8::from(*b))]
    pub is_created: bool,
}

impl Chunk for ConfigDataCreate {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x2432DA)
    }
}
