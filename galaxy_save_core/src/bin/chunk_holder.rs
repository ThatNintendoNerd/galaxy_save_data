use binrw::{BinRead, BinWrite, binrw};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A container for data blocks.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
pub struct BinaryDataChunkHolder<T>
where
    T: BinRead + BinWrite + ChunkHolder + 'static,
    T: for<'a> BinRead<Args<'a> = ()>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    /// The version number of the container.
    #[br(temp)]
    #[bw(calc = T::VERSION)]
    _version: u8,

    /// The number of stored data blocks.
    #[br(temp)]
    #[bw(calc = chunks.len() as u8)]
    chunk_num: u8,

    #[br(temp)]
    #[bw(calc = [0u8, 0u8])]
    _reserved: [u8; 2],

    /// The collection of data blocks.
    #[br(count = chunk_num as usize)]
    #[brw(pad_size_to = T::BUFFER_SIZE - size_of::<u32>())]
    pub chunks: Vec<T>,
}

/// A trait for types which must be represented as a data block container.
pub trait ChunkHolder {
    /// The size of the container's data buffer, in bytes.
    const BUFFER_SIZE: usize;

    /// The version number of the container.
    const VERSION: u8;
}
