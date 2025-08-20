//! Types associated with Hungry Luma state.

use binrw::binrw;
use galaxy_save_core::{bin::Chunk, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for Hungry Luma state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default)]
pub struct StarPieceAlmsStorage {
    /// The matrix of Star Bit counters, where each row corresponds to
    /// individual galaxies and the Comet Observatory, respectively, and each
    /// column corresponds to an individual Hungry Luma's consumed number of
    /// Star Bits.
    star_piece_num: [[u16; 8]; 2],
}

impl StarPieceAlmsStorage {
    /// Returns a reference to a Star Bit counter from the row a galaxy.
    pub fn galaxy(&self, index: usize) -> Option<&u16> {
        self.star_piece_num[0].get(index)
    }

    /// Returns a mutable reference to a Star Bit counter from a galaxy.
    pub fn galaxy_mut(&mut self, index: usize) -> Option<&mut u16> {
        self.star_piece_num[0].get_mut(index)
    }

    /// Returns a reference to a Star Bit counter from the Comet Observatory.
    pub fn astro_galaxy(&self, index: usize) -> Option<&u16> {
        self.star_piece_num[1].get(index)
    }

    /// Returns a reference to a Star Bit counter from the Comet Observatory.
    pub fn astro_galaxy_mut(&mut self, index: usize) -> Option<&mut u16> {
        self.star_piece_num[1].get_mut(index)
    }
}

impl Chunk for StarPieceAlmsStorage {
    fn hash_code() -> HashCode {
        let hash = HashCode::from("StarPieceAlmsStorage")
            .into_raw()
            .wrapping_shl(5);

        HashCode::from_raw(hash)
    }
}
