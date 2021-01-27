use crate::ident::{ElementIdent, ValueIdent};
use serde::ser::Error;
use std::{fmt, error};

#[derive(Debug, Clone)]
pub enum TychoSerializerError {
    InvalidValueType { found: ElementIdent, reason: String },
    ValueTypeMismatch { expected: ValueIdent, found: ValueIdent },
    NoKeyGiven,
    Custom { message: String }
}

impl Error for TychoSerializerError {
    fn custom<T>(msg: T) -> Self where
        T: fmt::Display {
        Self::Custom { message: msg.to_string() }
    }
}

impl fmt::Display for TychoSerializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TychoSerializerError::InvalidValueType { found, reason } => {
                f.write_str(&format!("Invalid type '{:?}' given when handling a value. {}", found, reason))
            }
            TychoSerializerError::ValueTypeMismatch { expected, found } => {
                f.write_str(&format!("Mismatch within given values. Expected '{:?}', found '{:?}' ", expected, found))
            }
            TychoSerializerError::NoKeyGiven => {
                f.write_str(&format!("No key was provided when serialising a map value."))
            }
            TychoSerializerError::Custom { message } => {
                f.write_str(&message)
            }
        }
    }
}
impl error::Error for TychoSerializerError {}