//! Data blocks storing gameplay data.

use binrw::binrw;
use galaxy_save_core::bin::{BinaryDataChunk, ChunkHolder};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod event_flag;
pub mod event_value;
pub mod galaxy;
pub mod player_status;
pub mod tico_fat;
pub mod world_map;

#[doc(inline)]
pub use event_flag::SaveDataStorageEventFlag;

#[doc(inline)]
pub use event_value::SaveDataStorageEventValue;

#[doc(inline)]
pub use galaxy::SaveDataStorageGalaxy;

#[doc(inline)]
pub use player_status::SaveDataStoragePlayerStatus;

#[doc(inline)]
pub use tico_fat::SaveDataStorageTicoFat;

#[doc(inline)]
pub use world_map::SaveDataStorageWorldMap;

/// The storage for some block of gameplay data.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum GameDataChunk {
    /// The data block dedicated to preserving player state.
    #[brw(magic = 0x504C4159u32 /* 'PLAY' */)]
    PlayerStatus(BinaryDataChunk<SaveDataStoragePlayerStatus>),

    /// The data block dedicated to preserving key-value pair state, where each
    /// value is a Boolean.
    #[brw(magic = 0x464C4731u32 /* 'FLG1' */)]
    EventFlag(BinaryDataChunk<SaveDataStorageEventFlag>),

    /// The data block dedicated to preserving Hungry Luma state.
    #[brw(magic = 0x53544631u32 /* 'STF1' */)]
    TicoFat(BinaryDataChunk<SaveDataStorageTicoFat>),

    /// The data block dedicated to preserving key-value pair state, where each
    /// value is a 16-bit unsigned integer.
    #[brw(magic = 0x564C4531u32 /* 'VLE1' */)]
    EventValue(BinaryDataChunk<SaveDataStorageEventValue>),

    /// The data block dedicated to preserving galaxy state.
    #[brw(magic = 0x47414C41u32 /* 'GALA' */)]
    Galaxy(BinaryDataChunk<SaveDataStorageGalaxy>),

    /// The data block dedicated to preserving World Map state.
    #[brw(magic = 0x5353574Du32 /* 'SSWM' */)]
    WorldMap(BinaryDataChunk<SaveDataStorageWorldMap>),
}

impl ChunkHolder for GameDataChunk {
    const BUFFER_SIZE: usize = 0xF80;
    const VERSION: u8 = 2;
}
