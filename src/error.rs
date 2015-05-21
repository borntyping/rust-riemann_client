//! Error variants

use std::io::Error as IoError;
use ::protobuf::error::ProtobufError;

#[derive(Debug)]
pub enum Error {
    Io(::std::io::Error),
    Protobuf(ProtobufError),
    Riemann(String)
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Io(err)
    }
}

impl From<ProtobufError> for Error {
    fn from(err: ProtobufError) -> Self {
        Error::Protobuf(err)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
