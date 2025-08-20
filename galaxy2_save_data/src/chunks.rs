//! Types defined from blocks of data.

pub mod config;
pub mod game;
pub mod sysconf;

#[doc(inline)]
pub use config::ConfigDataChunk;

#[doc(inline)]
pub use game::GameDataChunk;

#[doc(inline)]
pub use sysconf::SysConfigDataChunk;
