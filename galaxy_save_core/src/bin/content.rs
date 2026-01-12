use std::marker::PhantomData;

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::hash::HashCode16;

/// The dynamic reader/writer for the content of a data block.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinaryDataContentHeaderSerializer<T>
where
    T: HeaderSerializer,
{
    /// The number of stored field descriptors.
    #[br(temp)]
    #[bw(calc = attributes.len() as u16)]
    attribute_num: u16,

    /// The size of the serialized struct `T`, in bytes.
    #[br(temp)]
    #[bw(calc = T::data_size() as u16)]
    _data_size: u16,

    /// The collection of field descriptors.
    #[br(count = attribute_num as usize)]
    pub attributes: Vec<BinaryDataContentAttribute>,

    #[cfg_attr(feature = "serde", serde(skip))]
    phantom: PhantomData<T>,
}

impl<T> From<Vec<BinaryDataContentAttribute>> for BinaryDataContentHeaderSerializer<T>
where
    T: HeaderSerializer,
{
    fn from(attributes: Vec<BinaryDataContentAttribute>) -> Self {
        Self {
            attributes,
            phantom: PhantomData::<T>,
        }
    }
}

/// The descriptor for a field stored in a data block.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BinaryDataContentAttribute {
    /// The hashed name of the serialized field, truncated to the least significant 16 bits.
    pub key: HashCode16,

    /// The offset to the field in bytes, relative to the start of the serialized field data.
    pub offset: u16,
}

/// A trait for types which must support storing a `BinaryDataContentHeaderSerializer`.
pub trait HeaderSerializer: Sized {
    /// Creates a new `BinaryDataContentHeaderSerializer`.
    fn header_serializer() -> BinaryDataContentHeaderSerializer<Self>;

    /// Returns the serialized size of the `BinaryDataContentHeaderSerializer`, in bytes.
    fn header_size() -> usize;

    /// Returns the serialized size of `Self`, in bytes.
    fn data_size() -> usize;
}
