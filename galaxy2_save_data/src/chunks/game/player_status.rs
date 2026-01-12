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

    /// The collection of packed binary settings.
    #[header_serializer(name = "mFlag")]
    pub flag: SaveDataStoragePlayerStatusFlag,
    /*
    /// The number of scanned amiibo.
    #[header_serializer(name = "mAmiiboScanNum")]
    pub amiibo_scan_num: u8,

    /// The index of the Banktoad's active accessory.
    #[header_serializer(name = "mBankToadToolIndex")]
    pub bank_toad_tool_index: u8,

    /// Determines if Rosalina's Storybook was opened.
    #[header_serializer(name = "mIsPictureBookOpened")]
    pub is_picture_book_opened: bool,

    /// The number of cutscenes skipped.
    #[header_serializer(name = "mDemoSkipNum")]
    pub demo_skip_num: u16,

    /// The amount of time spent on the Original Soundtrack screen, in seconds.
    #[header_serializer(name = "mMusicPlaySeconds")]
    pub music_play_seconds: u16,

    /// The number of missions cleared.
    #[header_serializer(name = "mPlayNum")]
    pub play_num: u16,

    /// The number of missions cleared with Co-Star Mode enabled.
    #[header_serializer(name = "m2pNum")]
    pub p2_num: u16,

    /// The number of missions cleared with Luigi.
    #[header_serializer(name = "mLuigiNum")]
    pub luigi_num: u16,

    /// The amount of time taken to partially complete the game, in seconds.
    #[header_serializer(name = "mGameFinishTime")]
    pub game_finish_time: u32,

    /// The collection of packed binary settings for defeated boss state.
    #[header_serializer(name = "mBossesFinishedFlag")]
    pub bosses_finished_flag: SaveDataStoragePlayerStatusBossesFinishedFlag,

    // TODO: Unused?
    #[header_serializer(name = "mNpcConversationFlag")]
    pub npc_conversation_flag: u32,

    /// Determines if Assist Mode is enabled.
    #[header_serializer(name = "mIsAssistMode")]
    pub is_assist_mode: bool,
    */
}

impl Default for SaveDataStoragePlayerStatus {
    fn default() -> Self {
        Self {
            player_left: 4,
            stocked_star_piece_num: 0,
            stocked_coin_num: 0,
            last_1up_coin_num: 0,
            flag: Default::default(),
            /*
            amiibo_scan_num: 0,
            bank_toad_tool_index: 0,
            is_picture_book_opened: false,
            demo_skip_num: 0,
            music_play_seconds: 0,
            play_num: 0,
            p2_num: 0,
            luigi_num: 0,
            game_finish_time: 0,
            bosses_finished_flag: Default::default(),
            npc_conversation_flag: 0,
            is_assist_mode: false,
            */
        }
    }
}

impl Chunk for SaveDataStoragePlayerStatus {
    fn hash_code() -> HashCode {
        let hash = Self::data_size() as u32 + Self::header_size() as u32;

        HashCode::from_raw(hash)
    }
}

/// A collection of packed binary settings for player state.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct SaveDataStoragePlayerStatusFlag {
    /// Determines if Luigi is the current player character.
    pub player_luigi: bool,

    reserved: u7,
}

/*
/// A collection of packed binary settings for defeated boss state.
#[bitsize(32)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct SaveDataStoragePlayerStatusBossesFinishedFlag {
    /// Determines if the Whomp King was defeated.
    pub battan_king: bool,

    /// Determines if Gobblegut was defeated.
    pub belly_dragon: bool,

    /// Determines if Fiery Gobblegut was defeated.
    pub belly_dragon_lv2: bool,

    /// Determines if Glamdozer was defeated.
    pub boss_bussun: bool,

    /// Determines if King Lakitu was defeated.
    pub boss_jugem: bool,

    /// Determines if Bugaboom was defeated.
    pub boss_stink_bug: bool,

    /// Determines if Peewee Piranha was defeated.
    pub dino_packun_baby: bool,

    /// Determines if Dino Piranha was defeated.
    pub dino_packun_vs1: bool,

    /// Determines if Fiery Dino Piranha was defeated.
    pub dino_packun_vs2: bool,

    /// Determines if Major Burrows was defeated.
    pub dodoryu: bool,

    /// Determines if Rollodillo was defeated.
    pub king_tossin: bool,

    /// Determines if Bowser was defeated in Bowser's Lava Lair.
    pub koopa: bool,

    /// Determines if Bowser was defeated in Bowser's Gravity Gauntlet.
    pub koopa_lv2: bool,

    /// Determines if Bowser was defeated in Bowser's Fortified Fortress.
    pub koopa_lv3: bool,

    /// Determines if the Boomsday Machine was defeated.
    pub koopa_jr_castle: bool,

    /// Determines if Megahammer was defeated.
    pub koopa_jr_robot: bool,

    /// Determines if King Kaliente was defeated.
    pub ota_king: bool,

    /// Determines if Bouldergeist was defeated.
    pub polta: bool,

    /// Determines if Squizzard was defeated.
    pub sandy: bool,

    /// Determines if Digga-Leg was defeated.
    pub two_legs: bool,

    /// Determines if Sorbetti was defeated.
    pub yukkina: bool,

    /// Determines if Prince Pikante was defeated.
    pub ota_rock_tank: bool,

    /// Determines if the Mandibug Stack was defeated.
    pub stink_bug_parent: bool,

    reserved: u9,
}
*/
