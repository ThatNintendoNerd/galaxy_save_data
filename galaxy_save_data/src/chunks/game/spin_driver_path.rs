//! Types associated with Launch Star path state.

use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian, binread, binrw};
use galaxy_save_core::{bin::Chunk, hash::HashCode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for Launch Star path state.
#[binrw]
#[br(import(_data_size: usize))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SpinDriverPathStorage {
    /// The number of stored galaxies.
    #[br(temp)]
    #[bw(calc = galaxy.len() as u8)]
    galaxy_num: u8,

    /// The collection of associated galaxies.
    #[br(count = galaxy_num as usize)]
    galaxy: Vec<SpinDriverPathStorageGalaxy>,
}

impl SpinDriverPathStorage {
    /// Returns a reference to the [`SpinDriverPathStorageGalaxy`] corresponding to the key.
    pub fn get(&self, galaxy_name: impl Into<HashCode>) -> Option<&SpinDriverPathStorageGalaxy> {
        let galaxy_name = galaxy_name.into().into_raw() as u16;

        self.galaxy.iter().find(|v| v.galaxy_name == galaxy_name)
    }

    /// Returns a mutable reference to the [`SpinDriverPathStorageGalaxy`] corresponding to the key.
    pub fn get_mut(
        &mut self,
        galaxy_name: impl Into<HashCode>,
    ) -> Option<&mut SpinDriverPathStorageGalaxy> {
        let galaxy_name = galaxy_name.into().into_raw() as u16;

        self.galaxy
            .iter_mut()
            .find(|v| v.galaxy_name == galaxy_name)
    }
}

impl Chunk for SpinDriverPathStorage {
    fn hash_code() -> HashCode {
        HashCode::from_raw(0x12345679)
    }
}

/// A container for Launch Star path state in a galaxy.
#[binread]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SpinDriverPathStorageGalaxy {
    /// The hashed internal name of the galaxy, truncated to the least significant 16 bits.
    galaxy_name: u16,

    /// The size of the serialized struct, in bytes.
    #[br(temp)]
    _data_size: u16,

    /// The number of stored base missions.
    #[br(temp)]
    scenario_num: u8,

    #[br(temp)]
    _reserved: u8,

    /// The collection of associated base missions.
    #[br(count = scenario_num as usize)]
    pub scenario: Vec<SpinDriverPathStorageScenario>,
}

impl BinWrite for SpinDriverPathStorageGalaxy {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let start_pos = writer.stream_position()?;

        self.galaxy_name.write_options(writer, endian, ())?;

        let data_size_pos = writer.stream_position()?;
        writer.seek_relative(size_of::<u16>() as i64)?;

        let scenario_num = self.scenario.len() as u8;
        scenario_num.write(writer)?;

        writer.seek_relative(size_of::<u8>() as i64)?;

        for scenario in &self.scenario {
            scenario.write_options(writer, endian, ())?;
        }

        let data_size = (writer.stream_position()? - start_pos) as u16;
        writer.seek(SeekFrom::Start(data_size_pos))?;
        data_size.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::End(0))?;

        Ok(())
    }
}

/// A container for Launch Star path state in a base mission.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SpinDriverPathStorageScenario {
    /// The collection of Launch Star path states.
    pub one: Vec<SpinDriverPathStorageOne>,
}

impl SpinDriverPathStorageScenario {
    /// The bitmask for the index of the associated zone.
    const ZONE_ID_MASK: u8 = !Self::NEW_ZONE_ID_MASK;

    /// The bitmask for the magic number indicating a new zone.
    const NEW_ZONE_ID_MASK: u8 = 0b11000000;

    /// The value for terminating the loop.
    const TERM: u8 = b'\xFF';
}

impl BinRead for SpinDriverPathStorageScenario {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let start_pos = reader.stream_position()?;
        let expected_data_size = u16::read_options(reader, endian, ())? as u64;

        // The zone with the greatest number of Launch Stars is
        // `HellProminenceGalaxy` with a number of 12.
        let mut inner = Vec::with_capacity(12);
        let mut zone_id = None;

        loop {
            let value = u8::read(reader)?;

            if value == Self::TERM {
                break;
            }

            if value & Self::NEW_ZONE_ID_MASK == Self::NEW_ZONE_ID_MASK {
                zone_id = Some((value & Self::ZONE_ID_MASK) as i32);
                continue;
            }

            let zone_id = zone_id.expect("invalid byte sequence found");

            reader.seek_relative(-(size_of::<u8>() as i64))?;
            let one = SpinDriverPathStorageOne::read(reader, zone_id)?;

            inner.push(one);
        }

        let end_pos = reader.stream_position()?;
        let data_size = end_pos - start_pos;

        if data_size != expected_data_size {
            return Err(binrw::Error::AssertFail {
                pos: end_pos,
                message: format!(
                    "expected to read {expected_data_size} bytes, read {data_size} bytes",
                ),
            });
        }

        Ok(Self { one: inner })
    }
}

impl BinWrite for SpinDriverPathStorageScenario {
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let start_pos = writer.stream_position()?;
        writer.seek_relative(size_of::<u16>() as i64)?;

        if let Some(max_zone_id) = self.one.iter().map(|o| o.zone_id).max() {
            for zone_id in 0..=max_zone_id {
                let mut is_new_zone_id = true;

                for one in self.one.iter().filter(|o| o.zone_id == zone_id) {
                    if is_new_zone_id {
                        is_new_zone_id = false;

                        let value = zone_id as u8 & Self::ZONE_ID_MASK | Self::NEW_ZONE_ID_MASK;
                        value.write(writer)?;
                    }

                    one.write(writer)?;
                }
            }
        }

        let term = Self::TERM;
        term.write(writer)?;

        let data_size = (writer.stream_position()? - start_pos) as u16;
        writer.seek(SeekFrom::Start(start_pos))?;
        data_size.write_options(writer, endian, ())?;

        writer.seek(SeekFrom::End(0))?;

        Ok(())
    }
}

/// A container for the state of a Launch Star path.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct SpinDriverPathStorageOne {
    /// The index of the associated zone.
    pub zone_id: i32,

    /// The Launch Star object's `Obj_arg7` value.
    pub draw_range_index: i32,

    /// The percentage measuring how much of the path is traced.
    pub draw_range: f32,
}

impl SpinDriverPathStorageOne {
    /// The bitmask for the Launch Star object's `Obj_arg7` value.
    const DRAW_RANGE_INDEX_MASK: u8 = !(Self::FLAG_HI_MASK | Self::FLAG_LO_MASK);

    /// The bitmask for the Boolean determining if the path is completely traced.
    const FLAG_LO_MASK: u8 = 1 << (u8::BITS - 2);

    /// The bitmask for the Boolean determining if the path is incompletely traced.
    const FLAG_HI_MASK: u8 = 1 << (u8::BITS - 1);

    /// The scalar for mapping the percentage value to and from the unit interval.
    const DRAW_RANGE_FACTOR: f32 = 256.0;

    /// The minimum percentage value to be considered.
    const DRAW_RANGE_TOLERANCE: f32 = 0.001;

    /// Reads the data from the given reader.
    fn read<R: Read + Seek>(reader: &mut R, zone_id: i32) -> BinResult<Self> {
        let value = u8::read(reader)?;

        Ok(Self {
            zone_id,
            draw_range_index: (value & Self::DRAW_RANGE_INDEX_MASK) as i32,
            draw_range: match value {
                value if value & Self::FLAG_HI_MASK != 0 => {
                    let value = u8::read(reader)?;

                    if value != 0 {
                        value as f32 / Self::DRAW_RANGE_FACTOR
                    } else {
                        1.0
                    }
                }
                value if value & Self::FLAG_LO_MASK != 0 => 1.0,
                _ => 0.0,
            },
        })
    }

    /// Writes the data to the given writer.
    fn write<W: Write + Seek>(&self, writer: &mut W) -> BinResult<()> {
        if self.draw_range < Self::DRAW_RANGE_TOLERANCE {
            return Ok(());
        }

        let value = self.draw_range_index as u8 & Self::DRAW_RANGE_INDEX_MASK | Self::FLAG_HI_MASK;
        value.write(writer)?;

        let value = (self.draw_range * Self::DRAW_RANGE_FACTOR) as i32;

        if value <= 0 {
            return Ok(());
        }

        let value = value as u8;
        value.write(writer)?;

        Ok(())
    }
}
