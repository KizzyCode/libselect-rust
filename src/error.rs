//! Implements the error types

use ebacktrace::define_error;
use std::{
    io, error, result,
    fmt::{ self, Display, Formatter }
};


/// Creates a new variant
#[doc(hidden)]
#[macro_export] macro_rules! e {
    ($kind:expr, $($arg:tt)*) => ({ $crate::error::ErrorImpl::with_string($kind, format!($($arg)*)) })
}
/// Creates a new `ErrorImpl::InOutError` kind
#[doc(hidden)]
#[macro_export] macro_rules! eio {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InOutError, $($arg)*) });
}
/// Creates a new `ErrorImpl::InvalidValue` kind
#[doc(hidden)]
#[macro_export] macro_rules! einval {
    ($($arg:tt)*) => ({ e!($crate::error::ErrorKind::InvalidValue, $($arg)*) });
}


/// The error kind
#[derive(Debug)]
pub enum ErrorKind {
    /// An I/O-related error occurred
    InOutError,
    /// A value is invalid
    InvalidValue
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::InOutError => write!(f, "An I/O-error occurred"),
            Self::InvalidValue => write!(f, "A value is invalid")
        }
    }
}
impl error::Error for ErrorKind {
    /* Nothing to implement here */
}


// Define our custom error type
define_error!(ErrorImpl);
impl From<io::Error> for ErrorImpl<ErrorKind> {
    fn from(underlying: io::Error) -> Self {
        ErrorImpl::with_string(ErrorKind::InOutError, underlying)
    }
}


/// A nice typealias for our custom error
pub type Error = ErrorImpl<ErrorKind>;
/// A nice typealias for a `Result` with our custom error
pub type Result<T = ()> = result::Result<T, Error>;
