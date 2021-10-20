use core::{
    mem::MaybeUninit,
    slice,
};

use error::BadBytes;

#[cfg(test)]
mod tests;

mod error;

mod impls;

pub unsafe trait Vet {
    fn vet(bytes: &[u8]) -> Result<(), error::BadBytes>;
}

pub fn from_slice<T: Vet>(slice: &[u8]) -> Result<T, BadBytes> {
    if core::mem::size_of::<T>() != slice.len() {
        return Err(BadBytes::LengthMismatch {
            wanted: core::mem::size_of::<T>(),
            got: slice.len(),
        });
    }
    T::vet(slice)?;

    Ok(unsafe { slice.as_ptr().cast::<T>().read() })
}

#[cfg(feature = "std")]
pub fn from_read<T: Vet, R: std::io::Read>(r: &mut R) -> Result<T, error::FromReadError> {
    let mut dock = MaybeUninit::<T>::uninit();

    // Safety: uhhh
    let bytes: &mut [MaybeUninit<u8>] = unsafe {
        slice::from_raw_parts_mut(
            dock.as_mut_ptr() as *mut MaybeUninit<u8>,
            core::mem::size_of::<T>(),
        )
    };
    bytes.fill(MaybeUninit::new(0));

    // This is essentially array-assume-init through &mut
    // Safety: dubious
    let bytes: &mut [u8] = unsafe { &mut *(bytes as *mut _ as *mut [u8]) };

    r.read_exact(bytes)?;

    Ok(from_slice(bytes)?)
}
