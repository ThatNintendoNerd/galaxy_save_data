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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(DebugBits, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct ConfigDataMiiFlag {
    reserved: u1,

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

    /// The Rosalina head icon.
    Rosetta = 6,

    /// The Luma head icon.
    Tico = 7,
}
