//! Types associated with shared user file state.

use binrw::binrw;
use galaxy_save_core::{
    bin::{BinaryDataContentHeaderSerializer, Chunk, HeaderSerializer},
    hash::HashCode,
    time::Time,
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

    /// The timestamp representing when the player was encouraged to change their TV Type from 50 Hz to 60 Hz.
    #[header_serializer(name = "mTimeAnnounced")]
    pub time_announced: Time,

    /// The timestamp representing when the most recent message was sent to the Wii Message Board.
    #[header_serializer(name = "mTimeSent")]
    pub time_sent: Time,

    /// The number of bytes sent to the Wii Message Board from the date represented in [`time_sent`](#structfield.time_sent).
    #[header_serializer(name = "mSentBytes")]
    pub sent_bytes: u32,
}

impl Chunk for SysConfigData {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x1)
    }
}
