//! Types associated with player state.

use bilge::prelude::*;
use binrw::binrw;
use galaxy_save_core::{
    bin::{BinaryDataContentHeaderSerializer, Chunk, HeaderSerializer},
    hash::HashCode,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for player state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, HeaderSerializer)]
pub struct SaveDataStoragePlayerStatus {
    #[br(temp)]
    #[bw(calc = Self::header_serializer())]
    _serializer: BinaryDataContentHeaderSerializer<Self>,

    /// The number of remaining lives.
    #[header_serializer(name = "mPlayerLeft")]
    pub player_left: u8,

    /// The number of stashed Star Bits.
    #[header_serializer(name = "mStockedStarPieceNum")]
    pub stocked_star_piece_num: u16,

    /// The number of stashed coins.
    #[header_serializer(name = "mStockedCoinNum")]
    pub stocked_coin_num: u16,

    /// The most recent number of stashed coins to have awarded the player an extra life.
    #[header_serializer(name = "mLast1upCoinNum")]
    pub last_1up_coin_num: u16,

    /// The collection of binary settings.
    #[header_serializer(name = "mFlag")]
    pub flag: SaveDataStoragePlayerStatusFlag,
}

impl Default for SaveDataStoragePlayerStatus {
    fn default() -> Self {
        Self {
            player_left: 4,
            stocked_star_piece_num: 0,
            stocked_coin_num: 0,
            last_1up_coin_num: 0,
            flag: Default::default(),
        }
    }
}

impl Chunk for SaveDataStoragePlayerStatus {
    fn hash_code() -> HashCode {
        let hash = Self::data_size() as u32 + Self::header_size() as u32;

        HashCode::from_raw(hash)
    }
}

/// A collection of binary settings for player state.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        from = "ExpandedSaveDataStoragePlayerStatusFlag",
        into = "ExpandedSaveDataStoragePlayerStatusFlag"
    )
)]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct SaveDataStoragePlayerStatusFlag {
    /// Determines if Luigi is the current player character.
    pub player_luigi: bool,

    reserved: u7,
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ExpandedSaveDataStoragePlayerStatusFlag {
    player_luigi: bool,
}

#[cfg(feature = "serde")]
impl From<SaveDataStoragePlayerStatusFlag> for ExpandedSaveDataStoragePlayerStatusFlag {
    fn from(flag: SaveDataStoragePlayerStatusFlag) -> Self {
        Self {
            player_luigi: flag.player_luigi(),
        }
    }
}

#[cfg(feature = "serde")]
impl From<ExpandedSaveDataStoragePlayerStatusFlag> for SaveDataStoragePlayerStatusFlag {
    fn from(flag: ExpandedSaveDataStoragePlayerStatusFlag) -> Self {
        Self::new(flag.player_luigi)
    }
}
