//! Data blocks storing gameplay data.

use binrw::binrw;
use galaxy_save_core::bin::{BinaryDataChunk, ChunkHolder};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod event_flag;
pub mod event_value;
pub mod galaxy;
pub mod player_status;
pub mod spin_driver_path;
pub mod star_piece_alms;

#[doc(inline)]
pub use event_flag::GameEventFlagStorage;

#[doc(inline)]
pub use event_value::GameEventValueStorage;

#[doc(inline)]
pub use galaxy::GameDataAllGalaxyStorage;

#[doc(inline)]
pub use player_status::GameDataPlayerStatus;

#[doc(inline)]
pub use spin_driver_path::SpinDriverPathStorage;

#[doc(inline)]
pub use star_piece_alms::StarPieceAlmsStorage;

/// The storage for some block of gameplay data.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum GameDataChunk {
    /// The data block dedicated to preserving player state.
    #[brw(magic = 0x504C4159u32 /* 'PLAY' */)]
    PlayerStatus(BinaryDataChunk<GameDataPlayerStatus>),

    /// The data block dedicated to preserving key-value pair state, where each
    /// value is a Boolean.
    #[brw(magic = 0x464C4731u32 /* 'FLG1' */)]
    EventFlag(BinaryDataChunk<GameEventFlagStorage>),

    /// The data block dedicated to preserving Hungry Luma state.
    #[brw(magic = 0x50434531u32 /* 'PCE1' */)]
    StarPieceAlms(BinaryDataChunk<StarPieceAlmsStorage>),

    /// The data block dedicated to preserving Launch Star path state.
    #[brw(magic = 0x53504E31u32 /* 'SPN1' */)]
    SpinDriverPath(BinaryDataChunk<SpinDriverPathStorage>),

    /// The data block dedicated to preserving key-value pair state, where each
    /// value is a 16-bit unsigned integer.
    #[brw(magic = 0x564C4531u32 /* 'VLE1' */)]
    EventValue(BinaryDataChunk<GameEventValueStorage>),

    /// The data block dedicated to preserving galaxy state.
    #[brw(magic = 0x47414C41u32 /* 'GALA' */)]
    Galaxy(BinaryDataChunk<GameDataAllGalaxyStorage>),
}

impl ChunkHolder for GameDataChunk {
    const BUFFER_SIZE: usize = 0xF80;
    const VERSION: u8 = 1;
}
