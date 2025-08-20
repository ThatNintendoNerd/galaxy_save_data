use std::io::{Read, Seek, SeekFrom, Write};

use binrw::{BinRead, BinResult, BinWrite, Endian};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::hash::HashCode;

/// A block of data.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct BinaryDataChunk<T>
where
    T: BinRead + BinWrite + Chunk,
    T: for<'a> BinRead<Args<'a> = (usize,)>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    /*
    /// The hash digest identifying the data block.
    #[br(temp)]
    _hash_code: HashCode,

    /// The size of the data block, in bytes.
    #[br(temp)]
    _data_size: u32,
    */
    /// The wrapped value.
    pub inner: T,
}

impl<T> BinaryDataChunk<T>
where
    T: BinRead + BinWrite + Chunk,
    T: for<'a> BinRead<Args<'a> = (usize,)>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    /// The data alignment factor for little-endian architectures.
    const ALIGNMENT_LE: u64 = 4;
}

impl<T> BinRead for BinaryDataChunk<T>
where
    T: BinRead + BinWrite + Chunk,
    T: for<'a> BinRead<Args<'a> = (usize,)>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let hash_pos = reader.stream_position()?;
        let hash = HashCode::read_options(reader, endian, ())?;
        let expected_hash = T::hash_code();

        if hash != expected_hash {
            return Err(binrw::Error::AssertFail {
                pos: hash_pos,
                message: format!(
                    "expected hash digest {:#X}, found {:#X}",
                    expected_hash.into_raw(),
                    hash.into_raw()
                ),
            });
        }

        let header_size = size_of::<u32>() + size_of::<HashCode>() + size_of::<u32>();
        let expected_data_size = u32::read_options(reader, endian, ())? as usize;
        let inner = T::read_options(reader, endian, (expected_data_size - header_size,))?;
        let end_pos = match endian {
            Endian::Big => reader.stream_position()?,
            Endian::Little => reader
                .stream_position()?
                .next_multiple_of(Self::ALIGNMENT_LE),
        };

        reader.seek(SeekFrom::Start(end_pos))?;

        let data_size = (end_pos - (hash_pos - size_of::<u32>() as u64)) as usize;

        if data_size != expected_data_size {
            return Err(binrw::Error::AssertFail {
                pos: end_pos,
                message: format!(
                    "expected to read {expected_data_size} bytes, read {data_size} bytes",
                ),
            });
        }

        Ok(Self { inner })
    }
}

impl<T> BinWrite for BinaryDataChunk<T>
where
    T: BinRead + BinWrite + Chunk,
    T: for<'a> BinRead<Args<'a> = (usize,)>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    type Args<'a> = ();

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        _args: Self::Args<'_>,
    ) -> BinResult<()> {
        let start_pos = writer.stream_position()? - size_of::<u32>() as u64;
        let hash_code = T::hash_code();

        hash_code.write_options(writer, endian, ())?;

        let size_pos = writer.stream_position()?;

        writer.seek_relative(size_of::<u32>() as i64)?;

        self.inner.write_options(writer, endian, ())?;

        let end_pos = match endian {
            Endian::Big => writer.stream_position()?,
            Endian::Little => writer
                .stream_position()?
                .next_multiple_of(Self::ALIGNMENT_LE),
        };
        let data_size = (end_pos - start_pos) as u32;

        writer.seek(SeekFrom::Start(size_pos))?;
        data_size.write_options(writer, endian, ())?;
        writer.seek(SeekFrom::Start(end_pos))?;

        Ok(())
    }
}

/// A trait for types which must be represented as a data block.
pub trait Chunk {
    /// Returns the hash digest identifying the data block.
    fn hash_code() -> HashCode;
}
