#![macro_use]

use bencode;
use std::convert;
use std::io;
use bencode::{NumFromBencodeError, StringFromBencodeError};

#[macro_export]
macro_rules! get_optional_field {
    ($m:expr, $field:expr) => (
        match $m.get(&ByteString::from_str($field)) {
            Some(value) => FromBencode::from_bencode(value).expect("Failed to get value"),
            None => None,
        }
    )
}

#[macro_export]
macro_rules! get_field {
    ($m:expr, $field:expr) => (
        match $m.get(&ByteString::from_str($field)) {
            Some(value) => Ok(FromBencode::from_bencode(value).expect("Failed to get value")),
            None => Err(decoder_helper::DecodingError::MissingField),
        }
    )
}

#[derive(Debug)]
pub enum DecodingError {
    MissingField,
    NotADict,
    NotANumber(NumFromBencodeError),
    NotAString(StringFromBencodeError),
    IoError(io::Error),
    Error(bencode::streaming::Error),
}


impl convert::From<io::Error> for DecodingError {
    fn from(err: io::Error) -> DecodingError {
        DecodingError::IoError(err)
    }
}

impl convert::From<bencode::streaming::Error> for DecodingError {
    fn from(err: bencode::streaming::Error) -> DecodingError {
        DecodingError::Error(err)
    }
}

impl convert::From<bencode::NumFromBencodeError> for DecodingError {
    fn from(err: bencode::NumFromBencodeError) -> DecodingError {
        DecodingError::NotANumber(err)
    }
}

impl convert::From<bencode::StringFromBencodeError> for DecodingError {
    fn from(err: bencode::StringFromBencodeError) -> DecodingError {
        DecodingError::NotAString(err)
    }
}
