//! Basic binary data parsing utilities.

pub use galaxy_save_core_derive::HeaderSerializer;

mod chunk;
mod chunk_holder;
mod content;

pub use chunk::{BinaryDataChunk, Chunk};
pub use chunk_holder::{BinaryDataChunkHolder, ChunkHolder};
pub use content::{
    BinaryDataContentAttribute, BinaryDataContentHeaderSerializer, HeaderSerializer,
};
