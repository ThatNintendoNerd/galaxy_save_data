use binrw::{BinReaderExt, BinWriterExt, io::Cursor};

use super::*;

#[test]
fn read_buf_init() {
    let mut reader = Cursor::new(b"mario1\0\0\0\0\0\0");
    let value = reader.read_be::<FixedString12>().unwrap();

    assert_eq!(value.to_string().unwrap(), "mario1");
}

#[test]
fn read_buf_uninit() {
    let mut reader = Cursor::new(b"luigi1\0\xFF\xFF\xFF\xFF\xFF");
    let value = reader.read_be::<FixedString12>().unwrap();

    assert_eq!(value.to_string().unwrap(), "luigi1");
}

#[test]
fn read_buf_empty_init() {
    let mut reader = Cursor::new(b"\0\0\0\0\0\0\0\0");
    let value = reader.read_be::<FixedString12>().unwrap();

    assert_eq!(value.to_string().unwrap(), "");
}

#[test]
fn read_buf_empty_uninit() {
    let mut reader = Cursor::new(b"\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF");
    let value = reader.read_be::<FixedString12>().unwrap();

    assert_eq!(value.to_string().unwrap(), "");
}

#[test]
fn read_buf_missing_nul() {
    let mut reader = Cursor::new(b"user1");
    let value = reader.read_be::<FixedString<5>>();

    assert!(value.is_err());
}

#[test]
fn from_str_ok() {
    let s = "";
    let value = FixedString12::from_str(s).unwrap();
    assert_eq!(value.to_string().unwrap(), s);

    let s = "config1";
    let value = FixedString12::from_str(s).unwrap();
    assert_eq!(value.to_string().unwrap(), s);
}

#[test]
fn from_str_err() {
    let s = "buffer_overflow";
    let value = FixedString12::from_str(s);

    assert_eq!(value, Err(ParseFixedStringError::BufferOverflow));
}

#[test]
fn write_buf() {
    let value = FixedString12::from_str("sysconf").unwrap();
    let mut writer = Cursor::new(Vec::new());

    writer.write_be(&value).unwrap();

    assert_eq!(writer.into_inner(), b"sysconf\0\0\0\0\0");
}

#[test]
fn write_buf_empty() {
    let value = FixedString12::new();
    let mut writer = Cursor::new(Vec::new());

    writer.write_be(&value).unwrap();

    assert_eq!(writer.into_inner(), b"\0\0\0\0\0\0\0\0\0\0\0\0");
}
