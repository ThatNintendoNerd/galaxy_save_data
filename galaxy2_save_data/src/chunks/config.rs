//! Data blocks storing shared data between all associated user files.

use binrw::binrw;
use galaxy_save_core::bin::{BinaryDataChunk, ChunkHolder};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod create;
pub mod mii;
pub mod misc;

#[doc(inline)]
pub use create::ConfigDataCreate;

#[doc(inline)]
pub use mii::ConfigDataMii;

#[doc(inline)]
pub use misc::ConfigDataMisc;

/// The storage for some block of shared data between all associated user files.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum ConfigDataChunk {
    /// The data block dedicated to preserving user file creation state.
    #[brw(magic = 0x434F4E46u32 /* 'CONF' */)]
    Create(BinaryDataChunk<ConfigDataCreate>),

    /// The data block dedicated to preserving user file icon state.
    #[brw(magic = 0x4D494920u32 /* 'MII ' */)]
    Mii(BinaryDataChunk<ConfigDataMii>),

    /// The data block dedicated to preserving miscellaneous user file state.
    #[brw(magic = 0x4D495343u32 /* 'MISC' */)]
    Misc(BinaryDataChunk<ConfigDataMisc>),
}

impl ChunkHolder for ConfigDataChunk {
    const BUFFER_SIZE: usize = 0x60;
    const VERSION: u8 = 2;
}
