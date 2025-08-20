use super::*;

#[test]
fn from_chunks_exact() {
    let bytes = b"\x00\x00\x00\x02\x00\x00\x00\x13\x00\x00\xBE\x00";
    let checksum = Checksum::from_be_bytes(bytes);
    assert_eq!(checksum, Checksum::from_raw(0xBE1541E5));

    let bytes = b"\x02\x00\x00\x00\x13\x00\x00\x00\x00\xBE\x00\x00";
    let checksum = Checksum::from_le_bytes(bytes);
    assert_eq!(checksum, Checksum::from_raw(0xBE1541E5));
}

#[test]
fn from_chunks_inexact() {
    let bytes = b"\x00\x00\x00\x02\x00\x00\x00\x13\x00\x00\xBE\x00\x00";
    let checksum = Checksum::from_be_bytes(bytes);
    assert_eq!(checksum, Checksum::from_raw(0xBE1541E5));

    let bytes = b"\x02\x00\x00\x00\x13\x00\x00\x00\x00\xBE\x00\x00\x00";
    let checksum = Checksum::from_le_bytes(bytes);
    assert_eq!(checksum, Checksum::from_raw(0xBE1541E5));
}
