//! Error variants

use std::io::Error as IoError;

use protobuf::error::ProtobufError;

use super::proto::Msg;

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

impl From<Msg> for Error {
    fn from(msg: Msg) -> Self {
        Error::Riemann(msg.get_error().to_string())
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
