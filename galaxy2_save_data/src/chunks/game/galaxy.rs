//! Types associated with galaxy state.

use bilge::prelude::*;
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
pub struct SaveDataStorageGalaxy {
    /// The number of stored galaxy states.
    #[br(temp)]
    #[bw(calc = galaxy.len() as u16)]
    galaxy_num: u16,

    #[br(temp)]
    #[bw(calc = SaveDataStorageGalaxyStage::header_serializer())]
    _stage_serializer: BinaryDataContentHeaderSerializer<SaveDataStorageGalaxyStage>,

    #[br(temp)]
    #[bw(calc = SaveDataStorageGalaxyScenario::header_serializer())]
    _scenario_serializer: BinaryDataContentHeaderSerializer<SaveDataStorageGalaxyScenario>,

    /// The collection of galaxy states.
    #[br(count = galaxy_num as usize)]
    galaxy: Vec<SaveDataStorageGalaxyStage>,
}

impl SaveDataStorageGalaxy {
    /// Returns a reference to the [`SaveDataStorageGalaxyStage`] corresponding to the key.
    pub fn get(&self, galaxy_name: impl Into<HashCode>) -> Option<&SaveDataStorageGalaxyStage> {
        let galaxy_name = galaxy_name.into().into_raw() as u16;

        self.galaxy.iter().find(|v| v.galaxy_name == galaxy_name)
    }

    /// Returns a mutable reference to the [`SaveDataStorageGalaxyStage`] corresponding to the key.
    pub fn get_mut(
        &mut self,
        galaxy_name: impl Into<HashCode>,
    ) -> Option<&mut SaveDataStorageGalaxyStage> {
        let galaxy_name = galaxy_name.into().into_raw() as u16;

        self.galaxy
            .iter_mut()
            .find(|v| v.galaxy_name == galaxy_name)
    }
}

impl Chunk for SaveDataStorageGalaxy {
    fn hash_code() -> HashCode {
        let hash = SaveDataStorageGalaxyScenario::data_size() as u32
            + SaveDataStorageGalaxyStage::header_size() as u32
            + 2;

        HashCode::from_raw(hash)
    }
}

/// A container for the state of a galaxy.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, HeaderSerializer)]
pub struct SaveDataStorageGalaxyStage {
    /// The hashed internal name of the galaxy, truncated to the least significant 16 bits.
    #[header_serializer(name = "mGalaxyName")]
    galaxy_name: u16,

    /// The size of the serialized struct, in bytes.
    #[header_serializer(name = "mDataSize")]
    data_size: u16,

    /// The number of stored mission states.
    #[header_serializer(name = "mScenarioNum")]
    scenario_num: u8,

    /// The unit state on the World Map.
    #[header_serializer(name = "mGalaxyState")]
    pub galaxy_state: SaveDataStorageGalaxyState,

    /// The collection of packed binary settings.
    #[header_serializer(name = "mFlag")]
    pub flag: SaveDataStorageGalaxyFlag,

    /// The collection of mission states.
    #[br(count = scenario_num as usize)]
    #[header_serializer(skip)]
    pub scenario: Vec<SaveDataStorageGalaxyScenario>,
}

/// The unit state of a galaxy on the World Map.
#[binrw]
#[brw(repr(u8))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub enum SaveDataStorageGalaxyState {
    /// The galaxy is not available.
    #[default]
    Closed = 0,

    /// The galaxy is available and was not navigated over.
    New = 1,

    /// The galaxy is available and was navigated over.
    Opened = 2,
}

/// The collection of packed binary settings for a galaxy.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct SaveDataStorageGalaxyFlag {
    /// Determines if the Comet Medal was collected.
    pub tico_coin: bool,

    /// Determines if a Prankster Comet is in orbit.
    pub comet: bool,

    reserved: u6,
}

/// A container for the state of a mission.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, HeaderSerializer)]
pub struct SaveDataStorageGalaxyScenario {
    /// The number of lives lost.
    #[header_serializer(name = "mMissNum")]
    pub miss_num: u8,

    /// The best clear time, in frames.
    #[header_serializer(name = "mBestTime")]
    pub best_time: u32,

    /// The collection of packed binary settings.
    #[header_serializer(name = "mFlag")]
    pub flag: SaveDataStorageGalaxyScenarioFlag,
}

/// The collection of packed binary settings for a mission.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(SerializeBits, DeserializeBits))]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct SaveDataStorageGalaxyScenarioFlag {
    /// Determines if the Star was collected.
    pub power_star: bool,

    /// Determines if the Bronze Star was collected.
    pub bronze_star: bool,

    /// Determines if the mission was selected before.
    pub already_visited: bool,

    /// Determines if a ghost can appear.
    pub ghost_luigi: bool,

    /// Determines if Luigi has ever appeared on standby.
    pub intrusively_luigi: bool,

    reserved: u3,
}
