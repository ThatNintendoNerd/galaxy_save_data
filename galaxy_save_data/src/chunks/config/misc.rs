//! Types associated with miscellaneous user file state.

use bilge::prelude::*;
use binrw::binrw;
use galaxy_save_core::{bin::Chunk, hash::HashCode, time::OSTime};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for miscellaneous user file state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct ConfigDataMisc {
    /// The collection of packed binary settings.
    pub flag: ConfigDataMiscFlag,

    /// The timestamp representing when the user file was most recently saved.
    pub last_modified: OSTime,
}

impl Chunk for ConfigDataMisc {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x1)
    }
}

/// A collection of packed binary settings for miscellaneous user file state.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, FromBits)]
#[repr(transparent)]
pub struct ConfigDataMiscFlag {
    /// Determines if Mario was the most recently played character in the file.
    pub last_loaded_mario: bool,

    /// Determines if Mario has completed his part of the file.
    ///
    /// This flag is used in conjunction with [`complete_ending_luigi`](#method.complete_ending_luigi)
    /// to enable the Green Luma who will take the player character to the Grand Finale Galaxy.
    pub complete_ending_mario: bool,

    /// Determines if Luigi has completed his part of the file.
    ///
    /// This flag is used in conjunction with [`complete_ending_mario`](#method.complete_ending_mario)
    /// to enable the Green Luma who will take the player character to the Grand Finale Galaxy.
    pub complete_ending_luigi: bool,

    reserved: u5,
}

impl Default for ConfigDataMiscFlag {
    fn default() -> Self {
        Self::new(true, false, false)
    }
}
