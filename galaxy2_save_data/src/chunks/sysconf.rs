//! Data blocks storing shared data between all user files.

use binrw::binrw;
use galaxy_save_core::bin::{BinaryDataChunk, ChunkHolder};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod sys_config;

#[doc(inline)]
pub use sys_config::SysConfigData;

/// The storage for some block of shared data between all user files.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum SysConfigDataChunk {
    /// The data block dedicated to preserving shared state between all user files.
    #[brw(magic = 0x53595343u32 /* 'SYSC' */)]
    SysConfig(BinaryDataChunk<SysConfigData>),
}

impl ChunkHolder for SysConfigDataChunk {
    const BUFFER_SIZE: usize = 0x80;
    const VERSION: u8 = 2;
}
