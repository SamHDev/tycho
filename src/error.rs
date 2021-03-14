//! Error types returned from tycho marshall/unmarshall/serialise/deserialize processes.

use std::fmt;

#[derive(Debug)]
pub enum TychoError {
    Io(std::io::Error),
    InvalidIdent { found: u8, expecting: String },
    StringError(std::string::FromUtf8Error)
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
                => f.write_str(&format!("Found invalid ident byte '{}' when reading {}", found, expecting)),
        }
    }
}
impl std::error::Error for TychoError {}

pub type TychoResult<T> = Result<T, TychoError>;
pub type TychoStatus = TychoResult<()>;