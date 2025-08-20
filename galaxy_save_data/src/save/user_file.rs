use binrw::binrw;
use galaxy_save_core::{bin::BinaryDataChunkHolder, ptr::Ptr32, string::FixedString12};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::chunks::{config::ConfigDataChunk, game::GameDataChunk, sysconf::SysConfigDataChunk};

/// The descriptor for a user file.
#[binrw]
#[bw(import(offset: u32))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SaveDataUserFileInfo {
    /// The name of the user file.
    pub name: FixedString12,

    /// The pointer to the corresponding container of data blocks.
    #[br(args(name.to_str().unwrap()))]
    #[bw(args(offset, ()))]
    pub user_file: Ptr32<SaveDataUserFile>,
}

impl SaveDataUserFileInfo {
    /// Returns the serialized size of the `SaveDataUserFileInfo`, in bytes.
    pub(crate) const fn data_size() -> usize {
        size_of::<FixedString12>() + size_of::<u32>()
    }
}

/// The storage for some container of data blocks.
#[binrw]
#[br(import(user_file_name: &str))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub enum SaveDataUserFile {
    /// The container for blocks of gameplay data.
    #[br(pre_assert(user_file_name.starts_with("mario") || user_file_name.starts_with("luigi")))]
    GameData(BinaryDataChunkHolder<GameDataChunk>),

    /// The container for blocks of shared data between all associated user files.
    #[br(pre_assert(user_file_name.starts_with("config")))]
    ConfigData(BinaryDataChunkHolder<ConfigDataChunk>),

    /// The container for blocks of shared data between all user files.
    #[br(pre_assert(user_file_name == "sysconf"))]
    SysConfigData(BinaryDataChunkHolder<SysConfigDataChunk>),
}
