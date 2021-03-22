//! Error types returned from tycho marshall/unmarshall/serialise/deserialize processes.
use std::fmt;

use crate::ident::ElementIdent;
use crate::types::ident::ValueIdent;

#[derive(Debug)]
pub enum TychoError {
    Io(std::io::Error),
    InvalidIdent { found: u8, expecting: String },
    StringError(std::string::FromUtf8Error),
    Other(String),

    #[cfg(feature="partial_state")]
    OutdatedPointer,

    #[cfg(feature="serde_support")]
    InvalidKeyType { found: ElementIdent },
    #[cfg(feature="serde_support")]
    MismatchedType { found: ValueIdent, expected: ValueIdent }
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

pub type TychoResult<T> = Result<T, TychoError>;
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


