use super::*;

#[test]
fn test_on() {
    let array = [false, false, true, true, false, true, true, false];
    let bit_array = BitArray8::from(array);

    for i in 0..8 {
        assert_eq!(bit_array.test(i), array[i as usize]);
    }
}

#[test]
#[should_panic]
fn test_on_panic() {
    let bit_array = BitArray8::new();
    bit_array.test(8);
}

#[test]
fn test_set() {
    let mut bits = 0b00000000u8;
    let mut bit_array = BitArray8::from(bits);

    for i in 0..8 {
        bit_array.set(i);
        bits |= 1 << i;
        assert_eq!(bit_array, BitArray8::from(bits));
    }
}

#[test]
#[should_panic]
fn test_set_panic() {
    let mut bit_array = BitArray8::new();
    bit_array.set(8);
}

#[test]
fn test_clear() {
    let mut bits = 0b11111111u8;
    let mut bit_array = BitArray8::from(bits);

    for i in 0..8 {
        bit_array.clear(i);
        bits &= !(1 << i);
        assert_eq!(bit_array, BitArray8::from(bits));
    }
}

#[test]
#[should_panic]
fn test_clear_panic() {
    let mut bit_array = BitArray8::new();
    bit_array.clear(8);
}
