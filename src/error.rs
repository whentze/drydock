use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum BadBytes {
    LengthMismatch {
        wanted: usize,
        got: usize,
    },
    VetFailed,
}
impl fmt::Display for BadBytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "the bytes are bad. just really bad.")
    }
}

#[cfg(feature = "std")]
#[path = ""]
mod std_stuff {
    use super::*;
    use std::{error, io};

    impl error::Error for BadBytes {}

    #[derive(Debug)]
    pub enum FromReadError {
        Io(io::Error),
        BadBytes(BadBytes),
    }

    impl From<io::Error> for FromReadError {
        fn from(inner: io::Error) -> Self {
            Self::Io(inner)
        }
    }

    impl From<BadBytes> for FromReadError {
        fn from(inner: BadBytes) -> Self {
            Self::BadBytes(inner)
        }
    }
}
#[cfg(feature = "std")]
pub use std_stuff::*;
