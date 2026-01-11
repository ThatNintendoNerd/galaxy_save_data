//! Types associated with user file icon state.

use bilge::prelude::*;
use binrw::binrw;
use galaxy_save_core::{bin::Chunk, face::RFLCreateID, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for user file icon state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct ConfigDataMii {
    /// The collection of packed binary settings.
    pub flag: ConfigDataMiiFlag,

    /// The unique identifier of the Mii.
    pub mii_id: RFLCreateID,

    /// The icon of the user file.
    pub icon_id: ConfigDataMiiIcon,
}

impl Chunk for ConfigDataMii {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x2836E9)
    }
}

/// A collection of packed binary settings for user file icon state.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct ConfigDataMiiFlag {
    /// Unused.
    ///
    /// This flag is tested during deserialization of a [`ConfigDataMii`] if
    /// the [`icon_id`](struct.ConfigDataMii.html#structfield.icon_id) field is
    /// absent from the serialized data, updating the value of the field to
    /// equal [`Mii`](enum.ConfigDataMiiIcon.html#variant.mii) if set.
    /// However, the flag is never set and the field will never be absent
    /// during normal operation of the vanilla game, so neither conditional will
    /// ever pass without tampering.
    pub unk1: bool,

    /// Unused.
    ///
    /// This flag is set if the player chooses a Mii for their file, but is
    /// never tested or cleared.
    pub unk2: bool,

    reserved: u6,
}

/// An icon for a user file.
#[binrw]
#[brw(repr(u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub enum ConfigDataMiiIcon {
    /// The Mii head icon.
    Mii = 0,

    /// The Mario head icon.
    #[default]
    Mario = 1,

    /// The Luigi head icon.
    Luigi = 2,

    /// The Yoshi head icon.
    Yoshi = 3,

    /// The Toad head icon.
    Kinopio = 4,

    /// The Peach head icon.
    Peach = 5,
}
