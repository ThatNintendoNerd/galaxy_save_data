use super::*;

#[test]
fn from_ascii() {
    let hash = HashCode::from("LuigiTalkAfterRescued");
    assert_eq!(hash, HashCode::from_raw(0x8A57D763));

    let hash = HashCode::from("RosettaTalkAboutTico");
    assert_eq!(hash, HashCode::from_raw(0x41E64290));

    let hash = HashCode::from("RosettaTalkAfterNormalEnding");
    assert_eq!(hash, HashCode::from_raw(0xF41632AC));

    let hash = HashCode::from("ViewCompleteEnding");
    assert_eq!(hash, HashCode::from_raw(0xCF9AE9E5));

    let hash = HashCode::from("StarPieceCounterStop");
    assert_eq!(hash, HashCode::from_raw(0x6143AAE2));
}

#[test]
fn from_japanese_shift_jis() {
    // Strings in Rust are exclusively UTF-8, so treat them as byte slices for compatibility.
    let hash = HashCode::from(
        b"\x83\x6E\x83\x60\x83\x7D\x83\x8A\x83\x49\x8F\x89\x95\xCF\x90\x67".as_slice(),
    );
    assert_eq!(hash, HashCode::from_raw(0x878D7ABA));

    let hash = HashCode::from(
        b"\x83\x65\x83\x8C\x83\x54\x83\x7D\x83\x8A\x83\x49\x8F\x89\x95\xCF\x90\x67".as_slice(),
    );
    assert_eq!(hash, HashCode::from_raw(0xDD7E8C8E));

    let hash = HashCode::from(
        b"\x83\x7A\x83\x62\x83\x70\x81\x5B\x83\x7D\x83\x8A\x83\x49\x8F\x89\x95\xCF\x90\x67"
            .as_slice(),
    );
    assert_eq!(hash, HashCode::from_raw(0x9DF2184F));

    let hash = HashCode::from(
        b"\x83\x74\x83\x40\x83\x43\x83\x41\x83\x7D\x83\x8A\x83\x49\x8F\x89\x95\xCF\x90\x67"
            .as_slice(),
    );
    assert_eq!(hash, HashCode::from_raw(0xD43F8E5E));

    let hash = HashCode::from(
        b"\x83\x41\x83\x43\x83\x58\x83\x7D\x83\x8A\x83\x49\x8F\x89\x95\xCF\x90\x67".as_slice(),
    );
    assert_eq!(hash, HashCode::from_raw(0x7B8062E5));
}

#[test]
fn from_japanese_utf8() {
    let hash = HashCode::from("ハチマリオ初変身");
    assert_eq!(hash, HashCode::from_raw(0x7BB6DD3C));

    let hash = HashCode::from("テレサマリオ初変身");
    assert_eq!(hash, HashCode::from_raw(0x3385D79A));

    let hash = HashCode::from("ホッパーマリオ初変身");
    assert_eq!(hash, HashCode::from_raw(0xBC1AD7FD));

    let hash = HashCode::from("ファイアマリオ初変身");
    assert_eq!(hash, HashCode::from_raw(0x79CFDB55));

    let hash = HashCode::from("アイスマリオ初変身");
    assert_eq!(hash, HashCode::from_raw(0x5F3F0962));
}
