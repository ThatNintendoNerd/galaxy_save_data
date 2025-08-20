//! A pointer relative to the start of a data buffer.

use std::{
    io::{Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use binrw::{BinRead, BinResult, BinWrite, Endian};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A 32-bit pointer relative to the start of a data buffer.
pub type Ptr32<T> = Ptr<u32, T>;

/// A pointer relative to the start of a data buffer.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug)]
#[repr(transparent)]
pub struct Ptr<P, T> {
    inner: T,

    #[cfg_attr(feature = "serde", serde(skip))]
    phantom: PhantomData<P>,
}

impl<P, T> Ptr<P, T> {
    /// Creates a new `Ptr<P, T>`.
    pub fn new(value: T) -> Self {
        Self {
            inner: value,
            phantom: PhantomData::<P>,
        }
    }
}

impl<P, T> Deref for Ptr<P, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<P, T> DerefMut for Ptr<P, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<P, T> BinRead for Ptr<P, T>
where
    P: BinRead + Into<u64>,
    P: for<'a> BinRead<Args<'a> = ()>,
    T: BinRead,
{
    type Args<'a> = T::Args<'a>;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<Self> {
        let offset = P::read_options(reader, endian, P::Args::default())?;
        let pos = reader.stream_position()?;

        reader.seek(SeekFrom::Start(offset.into()))?;

        let value = T::read_options(reader, endian, args)?;

        reader.seek(SeekFrom::Start(pos))?;

        Ok(Self::new(value))
    }
}

impl<P, T> BinWrite for Ptr<P, T>
where
    P: BinWrite + Into<u64>,
    P: for<'a> BinWrite<Args<'a> = ()>,
    T: BinWrite,
{
    type Args<'a> = (P, T::Args<'a>);

    fn write_options<W: Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        let offset = args.0;

        offset.write_options(writer, endian, P::Args::default())?;

        let pos = writer.stream_position()?;

        writer.seek(SeekFrom::Start(offset.into()))?;
        self.inner.write_options(writer, endian, args.1)?;
        writer.seek(SeekFrom::Start(pos))?;

        Ok(())
    }
}
