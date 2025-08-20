//! Types associated with player state.

use binrw::binrw;
use galaxy_save_core::{bin::Chunk, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for player state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct GameDataPlayerStatus {
    /// The value of sequential progression through the story.
    ///
    /// Values are derived from a BCSV file hardcoded into the game's
    /// executable. It associates informal names of events with values. There
    /// does not appear to be any significance behind most values other than
    /// events which occur later in the game assuming a value greater than
    /// those before it.
    ///
    /// The BCSV file data converted to table format is as follows, where
    /// strings encoded in Shift JIS have been transcoded to UTF-8:
    ///
    /// | name | progress |
    /// | --- | --- |
    /// | ゲーム開始直後 | 0 |
    /// | クッパ襲来後 | 2 |
    /// | ピーチ城浮上後 | 5 |
    /// | チコガイドデモ終了 | 10 |
    /// | スピン権利 | 15 |
    /// | バトラー情報Ａ | 25 |
    /// | 天球儀レクチャー | 30 |
    /// | ギャラクシー移動レクチャー | 35 |
    /// | スターピースレクチャー | 40 |
    /// | クッパＪｒロボプラント発見 | 42 |
    /// | クッパスタープラント発見 | 45 |
    /// | クッパＪｒシッププラント発見 | 50 |
    /// | クッパダークマタープラント発見 | 55 |
    /// | クッパＪｒクリーチャープラント発見 | 60 |
    pub story_progress: u8,

    /// The number of stashed Star Bits.
    pub stocked_star_piece_num: u32,

    /// The number of remaining lives.
    pub player_left: u16,
}

impl Default for GameDataPlayerStatus {
    fn default() -> Self {
        Self {
            story_progress: 0,
            stocked_star_piece_num: 0,
            player_left: 4,
        }
    }
}

impl Chunk for GameDataPlayerStatus {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x27C90F)
    }
}
