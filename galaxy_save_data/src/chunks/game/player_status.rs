//! Types associated with player state.

// use bilge::prelude::*;
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
    /// does not appear to be any significance behind most `progress` values
    /// other than events which occur later in the game assuming a value greater
    /// than those before it.
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
    /*
    // TODO: Unused?
    unk1: u8,

    /// Determines if Rosalina's Storybook was opened.
    pub is_picture_book_opened: bool,

    /// The number of cutscenes skipped.
    pub demo_skip_num: u16,

    /// The amount of time spent on the Original Soundtrack screen, in seconds.
    pub music_play_seconds: u16,

    /// The number of missions cleared with Co-Star Mode enabled.
    pub p2_num: u16,

    /// The number of missions cleared with Luigi.
    pub luigi_num: u16,

    // TODO: Unused?
    unk7: u32,

    /// The collection of packed binary settings for defeated boss state.
    pub bosses_finished_flag: GameDataPlayerStatusBossesFinishedFlag,

    // TODO: Unused?
    unk9: u32,

    // TODO: Bit field, where the first bit determines if the game was partially completed.
    unk10: u8,

    /// The number of missions cleared.
    pub play_num: u16,
    */
}

impl Default for GameDataPlayerStatus {
    fn default() -> Self {
        Self {
            story_progress: 0,
            stocked_star_piece_num: 0,
            player_left: 4,
            /*
            unk1: 0,
            is_picture_book_opened: false,
            demo_skip_num: 0,
            music_play_seconds: 0,
            p2_num: 0,
            luigi_num: 0,
            unk7: 0,
            bosses_finished_flag: Default::default(),
            unk9: 0,
            unk10: 0,
            play_num: 0,
            */
        }
    }
}

impl Chunk for GameDataPlayerStatus {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x27C90F)
    }
}

/*
/// A collection of packed binary settings for defeated boss state.
#[bitsize(32)]
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct GameDataPlayerStatusBossesFinishedFlag {
    /// Determines if Topmaniac was defeated.
    pub boss_begoman: bool,

    /// Determines if Kamella was defeated in the Space Junk Galaxy.
    pub boss_kameck_vs1: bool,

    /// Determines if Kamella was defeated in the Deep Dark Galaxy.
    pub boss_kameck_vs2: bool,

    /// Determines if Bugaboom was defeated.
    pub boss_stink_bug: bool,

    /// Determines if Dino Piranha was defeated.
    pub dino_packun_vs1: bool,

    /// Determines if Fiery Dino Piranha was defeated.
    pub dino_packun_vs2: bool,

    /// Determines if Major Burrows was defeated.
    pub dodoryu: bool,

    /// Determines if Bowser was defeated in Bowser's Star Reactor.
    pub koopa_vs1: bool,

    /// Determines if Bowser was defeated in Bowser's Dark Matter Plant.
    pub koopa_vs2: bool,

    /// Determines if Bowser was defeated in Bowser's Galaxy Reactor.
    pub koopa_vs3: bool,

    /// Determines if King Kaliente was defeated in the Good Egg Galaxy.
    pub ota_king_vs1: bool,

    /// Determines if King Kaliente was defeated in Bowser Jr.'s Lava Reactor.
    pub ota_king_vs2: bool,

    /// Determines if Bouldergeist was defeated.
    pub polta: bool,

    /// Determines if Kingfin was defeated.
    pub skeletal_fish_boss: bool,

    /// Determines if Tarantox was defeated.
    pub tomb_spider: bool,

    /// Determines if Megaleg was defeated.
    pub tripod_boss: bool,

    /// Determines if Baron Brrr was defeated.
    pub ice_meramera_king: bool,

    /// Determines if the Mandibug Stack was defeated.
    pub stink_bug_parent: bool,

    reserved: u14,
}
*/
