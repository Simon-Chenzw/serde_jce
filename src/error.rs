use std;
use std::fmt::{self, Display};

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    UnknownJceType,

    NotEnoughtBytes,
    TrailingBytes,

    ErrorFieldTag,
    DuplicateFieldTag,
    DuplicateFieldTagName,

    WrongType,
    NeedLength,
    WrongLength,
    StringIsNotUtf8,

    IntTooBig,
    StringTooLong,
    BytesTooLong,
    SeqTooLong,
    MapTooLong,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, formatter)
    }
}

impl std::error::Error for Error {}
