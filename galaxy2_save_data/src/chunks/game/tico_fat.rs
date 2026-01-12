//! Types associated with Hungry Luma state.

use binrw::binrw;
use galaxy_save_core::{
    bin::Chunk,
    hash::{HashCode, HashCode16},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::chunks::game::SaveDataStorageWorldMap;

/// A container for Hungry Luma state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct SaveDataStorageTicoFat {
    /// The matrix of Star Bit counters, where each row corresponds to a world
    /// and each column corresponds to an individual Hungry Luma's consumed
    /// number of Star Bits.
    star_piece_num: [[u16; Self::PARTS_NUM]; SaveDataStorageWorldMap::WORLD_CAPACITY],

    /// The array of hashed internal galaxy names with satisfied coin-dependent
    /// Hungry Lumas, truncated to the least significant 16 bits.
    coin_galaxy_name: [HashCode16; Self::COIN_GALAXY_NAME_NUM],
}

impl SaveDataStorageTicoFat {
    /// The maximum number of Hungry Lumas that can be stored in any one world.
    const PARTS_NUM: usize = 6;

    /// The maximum number of hashed internal galaxy names with satisfied coin-
    /// dependent Hungry Lumas that can be stored.
    const COIN_GALAXY_NAME_NUM: usize = 16;

    /// Returns a reference to a Star Bit counter.
    pub fn star_piece_num(&self, world_no: usize, parts_index: usize) -> Option<&u16> {
        self.star_piece_num
            .get(world_no - 1)
            .and_then(|w| w.get(parts_index))
    }

    /// Returns a mutable reference to a Star Bit counter.
    pub fn star_piece_num_mut(&mut self, world_no: usize, parts_index: usize) -> Option<&mut u16> {
        self.star_piece_num
            .get_mut(world_no - 1)
            .and_then(|w| w.get_mut(parts_index))
    }

    /// Determines if a Hungry Luma in a galaxy was fed a satisfactory number of coins.
    pub fn is_coin_feed(&self, galaxy_name: impl Into<HashCode>) -> bool {
        let galaxy_name = HashCode16::from(galaxy_name.into());

        self.coin_galaxy_name.contains(&galaxy_name)
    }

    /// Registers a galaxy as having a Hungry Luma fed a satisfactory number of coins.
    pub fn on_coin_feed(&mut self, galaxy_name: impl Into<HashCode>) {
        let galaxy_name = HashCode16::from(galaxy_name.into());

        if self.coin_galaxy_name.contains(&galaxy_name) {
            return;
        }

        if let Some(hash) = self.coin_galaxy_name.iter_mut().find(|h| h.into_raw() == 0) {
            *hash = galaxy_name;
        }
    }

    /// Omits a galaxy from having a Hungry Luma fed a satisfactory number of coins.
    pub fn off_coin_feed(&mut self, galaxy_name: impl Into<HashCode>) {
        let galaxy_name = HashCode16::from(galaxy_name.into());
        let Some(position) = self
            .coin_galaxy_name
            .iter_mut()
            .position(|h| *h == galaxy_name)
        else {
            return;
        };

        self.coin_galaxy_name[position] = Default::default();

        for i in position..Self::COIN_GALAXY_NAME_NUM - 1 {
            if self.coin_galaxy_name[i + 1].into_raw() == 0 {
                break;
            }

            self.coin_galaxy_name.swap(i, i + 1);
        }
    }
}

impl Chunk for SaveDataStorageTicoFat {
    fn hash_code() -> HashCode {
        let hash = HashCode::from("SaveDataStorageTicoFat")
            .into_raw()
            .wrapping_add(0x120);

        HashCode::from_raw(hash)
    }
}
