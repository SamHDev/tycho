//! Error types returned from tycho marshall/unmarshall/serialise/deserialize processes.
use std::fmt;

use crate::ident::ElementIdent;
use crate::types::ident::ValueIdent;

#[derive(Debug)]
/// Error regarding a tycho process
pub enum TychoError {
    /// An `std::io::Error` was encountered while reading/writing to a or stream/buffer.
    Io(std::io::Error),
    /// An invalid identity was found when parsing a tycho ident prefix.
    InvalidIdent {
        /// The prefix byte found.
        found: u8,
        /// The name/type of the expected prefix.
        expecting: String
    },
    /// An error occurred while parsing a UTF-8 String from bytes.
    StringError(std::string::FromUtf8Error),

    /// An unspecified error.
    Other(String),

    #[cfg(feature="partial_state")]
    /// A pointer was referenced, but is no-longer valid as the data may have changed.
    OutdatedPointer,

    #[cfg(feature="serde_support")]
    /// A key was mismatched when handling serde.
    InvalidKeyType {
        /// The type of element found.
        found: ElementIdent
    },
    #[cfg(feature="serde_support")]
    /// A type was mismatched when handling serde.
    MismatchedType {
        /// The type of element found.
        found: ValueIdent,
        /// The type of element expected.
        expected: ValueIdent
    },

}

impl From<std::io::Error> for TychoError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

pub(crate) trait FromResult<T> {
    fn from(r: Result<T,std::io::Error>) -> Self;
}

impl<T> FromResult<T> for TychoResult<T> {
    fn from(r: Result<T,std::io::Error>) -> Self {
        match r {
            Ok(x) => Ok(x),
            Err(e) => Err(TychoError::Io(e))
        }
    }
}

pub(crate) fn parse_io<T>(r: Result<T,std::io::Error>) -> TychoResult<T> {
    match r {
        Ok(x) => Ok(x),
        Err(e) => Err(TychoError::Io(e))
    }
}



impl fmt::Display for TychoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TychoError::Io(x) => x.fmt(f),
            TychoError::StringError(x) => x.fmt(f),
            TychoError::InvalidIdent { found, expecting }
                => f.write_str(&format!("Found invalid ident byte '{}' when reading {}",
                                        found, expecting)),
            TychoError::Other(x) =>
                f.write_str(x),

            #[cfg(feature="partial_state")]
            TychoError::OutdatedPointer =>
                f.write_str("Failed to reference partial pointer, outdated in respect \
                to reader."),

            #[cfg(feature="serde_support")]
            TychoError::InvalidKeyType { found } =>
                f.write_str(&format!("Invalid key type while serializing structure: found type\
                 {:?}", found)),

            #[cfg(feature="serde_support")]
            TychoError::MismatchedType { found, expected } =>
                f.write_str(&format!("Mismatched type while serializing structure of type\
                 {:?}: found type {:?}", expected, found)),
        }
    }
}
impl std::error::Error for TychoError {}

/// A result which errors with `TychoError` (`Result<T, TychoError>`)
pub type TychoResult<T> = Result<T, TychoError>;

/// A unit result which errors with `TychoError` (`Result<(), TychoError>`)
pub type TychoStatus = TychoResult<()>;


#[cfg(feature="serde_support")]
impl serde::ser::Error for TychoError {
    fn custom<T>(msg: T) -> Self where
        T: std::fmt::Display {
        Self::Other(msg.to_string())
    }
}

#[cfg(feature="serde_support")]
impl serde::de::Error for TychoError {
    fn custom<T>(msg: T) -> Self where
        T: std::fmt::Display {
        Self::Other(msg.to_string())
    }
}


