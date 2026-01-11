//! Basic Mii utilities.

use binrw::binrw;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A unique identifier for a Mii.
///
/// # Format
///
/// The bit layout of an `RFLCreateID` under a big-endian byte order is as
/// follows:
///
/// | **Bit Length**  | 4     | 28        | 8                       | 8                          | 8                         | 8                         |
/// | --------------- | ----- | --------- | ----------------------- | -------------------------- | ------------------------- | ------------------------- |
/// | **Designation** | Flags | Timestamp | Checksum of MAC Address | Fourth Byte of MAC Address | Fifth Byte of MAC Address | Sixth Byte of MAC Address |
///
/// ## Flags
///
/// - `0b0001` - Unused.
/// - `0b0010` - Unknown.
///   - This flag is checked by the `RFLiIsTemporaryID` function.
///   - This flag is set by the `RFLiSetTemporaryID` function.
/// - `0b0100` - Determines if the Mii was created on another system.
///   - If this flag is set, the Mii will wear blue pants.
/// - `0b1000` - Determines if the Mii was not distributed by Nintendo.
///   - If this flag is set, the Mii will not wear yellow pants.
///
/// ## Timestamp
///
/// The timestamp represents the beginning of the Mii character's creation,
/// specifically after their gender was selected.
///
/// ### Computation
///
/// To compute the timestamp, the return value from the `OSGetTime`[^1]
/// function is first subtracted by the value of the `scStartTime`[^2]
/// variable. The difference is then divided by the evaluated value of
/// `OSSecondsToTicks(4)`[^3]. Finally, the quotient is masked to 28 bits,
/// allotting space for the flags in the resulting bit field.
///
/// The pseudocode for the above explanation is provided as follows:
///
/// ```ignore
/// let timestamp = (OSGetTime() - scStartTime) / OSSecondsToTicks(4) & 0xFFFFFFF;
/// ```
///
/// [^1]: `OSGetTime` is a function from the OS library in the Revolution SDK
/// which returns the current value of the time base register.
/// [^2]: `scStartTime` is a variable from the Revolution Face Library which
/// represents the epoch for Mii creation. It is equal to `11505369649262175`,
/// or the approximate beginning of the year 2006 as an `OSTime`.
/// [^3]: `OSSecondsToTicks` is a macro from the OS library in the Revolution
/// SDK which converts a given number of seconds into an `OSTime`.
///
/// ## MAC Address
///
/// The last 32 bits are related to the Wii console's MAC address.
///
/// ### Checksum
///
/// Given a MAC address, the checksum algorithm calculates the sum of the first
/// three bytes. If any of these bytes do not equal the corresponding byte from
/// the `scFirstMakerCode`[^4] array, the most significant bit in the resulting
/// sum is discarded.
///
/// The pseudocode for the above explanation is provided as follows:
///
/// ```ignore
/// let mac_addr = RFLiGetMacAddr();
/// let code = &mac_addr[..3];
/// let mut sum: u8 = code.iter().sum();
///
/// if code != scFirstMakerCode {
///     sum &= !0x80;
/// }
/// ```
///
/// [^4]: `scFirstMakerCode` is an array from the Revolution Face Library which
/// represents the first three bytes of a potential MAC address. It is equal to
/// `[0x00, 0x17, 0xAB]`, or the first OUI registered to Nintendo Co., Ltd for
/// use with the Wii console.
#[binrw]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct RFLCreateID {
    inner: [u8; 0x8],
}
