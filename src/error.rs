use std;
use trackable::error::{ErrorKind as TrackableErrorKind, ErrorKindExt, TrackableError};

use entity::object::ObjectVersion;

/// クレート固有の`Error`型。
#[derive(Debug, Clone, TrackableError, Serialize, Deserialize)]
pub struct Error(TrackableError<ErrorKind>);
impl From<std::io::Error> for Error {
    fn from(f: std::io::Error) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}
impl From<std::ffi::NulError> for Error {
    fn from(f: std::ffi::NulError) -> Self {
        ErrorKind::Other.cause(f).into()
    }
}
impl From<std::num::ParseIntError> for Error {
    fn from(f: std::num::ParseIntError) -> Self {
        ErrorKind::InvalidInput.cause(f).into()
    }
}

/// エラーの種類。
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorKind {
    InvalidInput,
    Unavailable,
    Timeout,
    NotLeader,
    Unexpected(Option<ObjectVersion>),
    Other,
}
impl TrackableErrorKind for ErrorKind {}
