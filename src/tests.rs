use core::num::{NonZeroU8, NonZeroI64};

use crate::{error::BadBytes, from_slice};

#[test]
fn u8() {
    assert_eq!(from_slice::<u8>(&[42]), Ok(42));
}

#[test]
fn i32() {
    assert_eq!(from_slice::<i32>(&[0x12, 0x34, 0x56, 0x78]), Ok(0x78563412));
}

#[test]
fn bool() {
    assert_eq!(from_slice::<bool>(&[0x00]), Ok(false));
    assert_eq!(from_slice::<bool>(&[0x01]), Ok(true));
    for byte in 2..=255 {
        assert_eq!(from_slice::<bool>(&[byte]), Err(BadBytes::VetFailed));
    }
}

#[test]
fn nonzero_u8() {
    assert_eq!(from_slice::<NonZeroU8>(&[0]), Err(BadBytes::VetFailed));
    assert_eq!(from_slice::<NonZeroU8>(&[5]), Ok(NonZeroU8::new(5).unwrap()));
}

#[test]
fn nonzero_i64() {
    assert_eq!(from_slice::<NonZeroI64>(&[0; 8]), Err(BadBytes::VetFailed));
    assert_eq!(from_slice::<NonZeroI64>(&[1; 8]), Ok(NonZeroI64::new(0x0101010101010101).unwrap()));
}

#[test]
fn wrong_len() {
    assert_eq!(
        from_slice::<f32>(&[0, 0]),
        Err(BadBytes::LengthMismatch { wanted: 4, got: 2 })
    );
}
