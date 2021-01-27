use serde::de;
use std::fmt;
use serde::__private::Formatter;
use crate::ident::{ValueIdent, ElementIdent};
use crate::decode::DecodeError;

#[derive(Debug)]
pub enum TychoDeserializeError {
    Custom { message: String },
    ExpectingValueType { expecting: ValueIdent, found: ValueIdent },
    ExpectingElementType { expecting: ElementIdent, found: ElementIdent },
    KeyError,
    DecodeError(DecodeError)
}


impl de::Error for TychoDeserializeError {
    fn custom<T>(msg: T) -> Self where
        T: fmt::Display {
        TychoDeserializeError::Custom { message: msg.to_string() }
    }
}

impl fmt::Display for TychoDeserializeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            TychoDeserializeError::Custom { message } => {
                f.write_str(&message)
            },
            TychoDeserializeError::KeyError => {
                f.write_str("Failed to resolve key when handling key-value pair")
            },
            TychoDeserializeError::ExpectingValueType { expecting, found } => {
                f.write_str(&format!("Expecting value of type '{:?}', found value type '{:?}'", expecting, found))
            },
            TychoDeserializeError::ExpectingElementType { expecting, found } => {
                f.write_str(&format!("Expecting element of type '{:?}', found element type '{:?}'", expecting, found))
            },
            TychoDeserializeError::DecodeError(e) => {
                f.write_str(&format!("Error while decoding bytes: {:?}", e))
            }
        }
    }
}

impl std::error::Error for TychoDeserializeError {}