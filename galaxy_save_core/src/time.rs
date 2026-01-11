//! Basic time utilities.

/// A general-purpose 64-bit signed time.
///
/// # Platform Differences
///
/// Time will be represented differently depending on the origin of the save
/// file.
///
/// ## Wii
///
/// This type corresponds to `OSTime` from the OS library in the Revolution
/// SDK, which represents the number of ticks since the Revolution OS epoch;
/// 2000-01-01 00:00:00.
///
/// This also applies to NVIDIA Shield TV because the game runs under an
/// emulator for Nintendo GameCube and Wii software.
///
/// ## Nintendo Switch
///
/// This type corresponds to `PosixTime` from the time library in `nn`, which
/// represents the number of non-leap seconds since the Unix epoch;
/// 1970-01-01 00:00:00 UTC.
pub type Time = i64;
