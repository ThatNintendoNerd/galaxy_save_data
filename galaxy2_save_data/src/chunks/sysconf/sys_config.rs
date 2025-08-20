//! Types associated with shared user file state.

use binrw::binrw;
use galaxy_save_core::{
    bin::{BinaryDataContentHeaderSerializer, Chunk, HeaderSerializer},
    hash::HashCode,
    time::OSTime,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for shared state between all user files.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, HeaderSerializer)]
pub struct SysConfigData {
    #[br(temp)]
    #[bw(calc = Self::header_serializer())]
    _serializer: BinaryDataContentHeaderSerializer<Self>,

    /// Determines if the player was encouraged to change their TV Type from 50 Hz to 60 Hz.
    #[br(map = |b: u8| b != 0)]
    #[bw(map = |b| u8::from(*b))]
    #[header_serializer(name = "mIsEncouragePal60")]
    pub is_encourage_pal60: bool,

    /// The timestamp representing when the most recent message was sent to the Wii Message Board.
    #[header_serializer(name = "mTimeSent")]
    pub time_sent: OSTime,

    /// The number of bytes sent to the Wii Message Board from the date represented in [`time_sent`](#structfield.time_sent).
    #[header_serializer(name = "mSentBytes")]
    pub sent_bytes: u32,

    /// The number of Star Bits stored with the banktoad.
    #[header_serializer(name = "mBankStarPieceNum")]
    pub bank_star_piece_num: u16,

    /// The greatest number of Star Bits stored with the banktoad.
    #[header_serializer(name = "mBankStarPieceMax")]
    pub bank_star_piece_max: u16,

    /// The number of extra lives from another user file attached to a letter from Rosalina.
    #[header_serializer(name = "mGiftedPlayerLeft")]
    pub gifted_player_left: u8,

    /// The sender of extra lives' hashed user file name.
    #[header_serializer(name = "mGiftedFileNameHash")]
    pub gifted_file_name_hash: u16,
}

impl Chunk for SysConfigData {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x3)
    }
}
