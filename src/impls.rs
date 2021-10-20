use crate::{Vet, error::BadBytes};

unsafe impl Vet for bool {
    fn vet(bytes: &[u8]) -> Result<(), crate::error::BadBytes> {
        match bytes[0] {
            0 | 1 => Ok(()),
            _ => Err(BadBytes::VetFailed),
        }
    }
}

macro_rules! all_pass_impls {
    ($type: ty) => {
        unsafe impl Vet for $type {
            fn vet(_: &[u8]) -> Result<(), $crate::error::BadBytes> {
                Ok(())
            }
        }
    };
    ($type: ty, $($types:ty),+) => {
        all_pass_impls!($type);
        all_pass_impls!($($types),+);
    };
}

all_pass_impls!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    ()
);

macro_rules! nonzero_impls {
    ($type: ty) => {
        unsafe impl Vet for $type {
            fn vet(bytes: &[u8]) -> Result<(), $crate::error::BadBytes> {
                if bytes[..(core::mem::size_of::<Self>())].iter().all(|&b| b != 0u8){
                    Ok(())
                } else {
                    Err(BadBytes::VetFailed)
                }
            }
        }
    };
    ($type: ty, $($types:ty),+) => {
        nonzero_impls!($type);
        nonzero_impls!($($types),+);
    };
}

nonzero_impls!(
    std::num::NonZeroU8,
    std::num::NonZeroU16,
    std::num::NonZeroU32,
    std::num::NonZeroU64,
    std::num::NonZeroU128,
    std::num::NonZeroUsize,
    std::num::NonZeroI8,
    std::num::NonZeroI16,
    std::num::NonZeroI32,
    std::num::NonZeroI64,
    std::num::NonZeroI128,
    std::num::NonZeroIsize
);