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
    pub fn get(&self, key: impl Into<HashCode>) -> Option<&SaveDataStorageGalaxyStage> {
        let key = key.into().into_raw() as u16;

        self.galaxy.iter().find(|v| v.galaxy_name == key)
    }

    /// Returns a mutable reference to the [`SaveDataStorageGalaxyStage`] corresponding to the key.
    pub fn get_mut(&mut self, key: impl Into<HashCode>) -> Option<&mut SaveDataStorageGalaxyStage> {
        let key = key.into().into_raw() as u16;

        self.galaxy.iter_mut().find(|v| v.galaxy_name == key)
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

    /// The collection of binary settings.
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

/// The collection of binary settings for a galaxy.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        from = "ExpandedSaveDataStorageGalaxyFlag",
        into = "ExpandedSaveDataStorageGalaxyFlag"
    )
)]
#[derive(DebugBits, Clone, Copy, DefaultBits, FromBits)]
#[repr(transparent)]
pub struct SaveDataStorageGalaxyFlag {
    /// Determines if the Comet Medal was collected.
    pub tico_coin: bool,

    /// Determines if a Prankster Comet is in orbit.
    pub comet: bool,

    reserved: u6,
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ExpandedSaveDataStorageGalaxyFlag {
    tico_coin: bool,
    comet: bool,
}

#[cfg(feature = "serde")]
impl From<SaveDataStorageGalaxyFlag> for ExpandedSaveDataStorageGalaxyFlag {
    fn from(flag: SaveDataStorageGalaxyFlag) -> Self {
        Self {
            tico_coin: flag.tico_coin(),
            comet: flag.comet(),
        }
    }
}

#[cfg(feature = "serde")]
impl From<ExpandedSaveDataStorageGalaxyFlag> for SaveDataStorageGalaxyFlag {
    fn from(flag: ExpandedSaveDataStorageGalaxyFlag) -> Self {
        Self::new(flag.tico_coin, flag.comet)
    }
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

    /// The collection of binary settings.
    #[header_serializer(name = "mFlag")]
    pub flag: SaveDataStorageGalaxyScenarioFlag,
}

/// The collection of binary settings for a mission.
#[bitsize(8)]
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(
        from = "ExpandedSaveDataStorageGalaxyScenarioFlag",
        into = "ExpandedSaveDataStorageGalaxyScenarioFlag"
    )
)]
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

#[cfg(feature = "serde")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
struct ExpandedSaveDataStorageGalaxyScenarioFlag {
    power_star: bool,
    bronze_star: bool,
    already_visited: bool,
    ghost_luigi: bool,
    intrusively_luigi: bool,
}

#[cfg(feature = "serde")]
impl From<SaveDataStorageGalaxyScenarioFlag> for ExpandedSaveDataStorageGalaxyScenarioFlag {
    fn from(flag: SaveDataStorageGalaxyScenarioFlag) -> Self {
        Self {
            power_star: flag.power_star(),
            bronze_star: flag.bronze_star(),
            already_visited: flag.already_visited(),
            ghost_luigi: flag.ghost_luigi(),
            intrusively_luigi: flag.intrusively_luigi(),
        }
    }
}

#[cfg(feature = "serde")]
impl From<ExpandedSaveDataStorageGalaxyScenarioFlag> for SaveDataStorageGalaxyScenarioFlag {
    fn from(flag: ExpandedSaveDataStorageGalaxyScenarioFlag) -> Self {
        Self::new(
            flag.power_star,
            flag.bronze_star,
            flag.already_visited,
            flag.ghost_luigi,
            flag.intrusively_luigi,
        )
    }
}
