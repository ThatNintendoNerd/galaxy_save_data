//! Types associated with galaxy state.

use binrw::binrw;
use galaxy_save_core::{
    bin::{BinaryDataContentHeaderSerializer, Chunk, HeaderSerializer},
    hash::HashCode,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for galaxy state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct GameDataAllGalaxyStorage {
    /// The number of stored galaxy states.
    #[br(temp)]
    #[bw(calc = galaxy.len() as u16)]
    galaxy_num: u16,

    #[br(temp)]
    #[bw(calc = GameDataSomeGalaxyStorage::header_serializer())]
    _serializer: BinaryDataContentHeaderSerializer<GameDataSomeGalaxyStorage>,

    /// The collection of galaxy states.
    #[br(count = galaxy_num as usize)]
    galaxy: Vec<GameDataSomeGalaxyStorage>,
}

impl GameDataAllGalaxyStorage {
    /// Returns a reference to the [`GameDataSomeGalaxyStorage`] corresponding to the given key.
    pub fn get(&self, galaxy_name: impl Into<HashCode>) -> Option<&GameDataSomeGalaxyStorage> {
        let galaxy_name = galaxy_name.into().into_raw() as u16;

        self.galaxy.iter().find(|v| v.galaxy_name == galaxy_name)
    }

    /// Returns a mutable reference to the [`GameDataSomeGalaxyStorage`] corresponding to the given key.
    pub fn get_mut(
        &mut self,
        galaxy_name: impl Into<HashCode>,
    ) -> Option<&mut GameDataSomeGalaxyStorage> {
        let galaxy_name = galaxy_name.into().into_raw() as u16;

        self.galaxy
            .iter_mut()
            .find(|v| v.galaxy_name == galaxy_name)
    }
}

impl Chunk for GameDataAllGalaxyStorage {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0xBF0640EE)
    }
}

/// A container for the state of a galaxy.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, HeaderSerializer)]
pub struct GameDataSomeGalaxyStorage {
    /// The hashed internal name of the galaxy, truncated to the least significant 16 bits.
    #[header_serializer(name = "mGalaxyName")]
    galaxy_name: u16,

    /// The flags representing the Star collection status for each mission.
    #[header_serializer(name = "mPowerStarFlag")]
    pub power_star_flag: u8,

    /// The flags representing the selection status for each base mission.
    #[header_serializer(name = "mFirstPlayFlag")]
    pub first_play_flag: u8,

    /// The greatest number of collected coins for each mission.
    #[header_serializer(name = "mMaxCoinNum")]
    pub max_coin_num: [u16; u8::BITS as usize],
}
