use crate::ident::{ValueIdent, Ident};
use crate::Value;
use crate::encode::util::{prefix_bytes, join_bytes};
use crate::encode::string::{encode_string, encode_term_string};
use crate::encode::length::encode_variable_length;

pub trait ValueEncoder {
    fn ident(&self) -> ValueIdent;
    fn body(&self) -> Vec<u8>;
    fn encode(&self) -> Vec<u8> { prefix_bytes(self.ident().ident(), self.body()) }
}

impl ValueEncoder for Value {
    fn ident(&self) -> ValueIdent {
        match &self {
            Value::Boolean(_) => ValueIdent::Boolean,
            Value::Unsigned8(_) => ValueIdent::Unsigned8,
            Value::Unsigned16(_) => ValueIdent::Unsigned16,
            Value::Unsigned32(_) => ValueIdent::Unsigned32,
            Value::Unsigned64(_) => ValueIdent::Unsigned64,
            Value::Unsigned128(_) => ValueIdent::Unsigned128,
            Value::Signed8(_) => ValueIdent::Signed8,
            Value::Signed16(_) => ValueIdent::Signed16,
            Value::Signed32(_) => ValueIdent::Signed32,
            Value::Signed64(_) => ValueIdent::Signed64,
            Value::Signed128(_) => ValueIdent::Signed128,
            Value::Float32(_) => ValueIdent::Float32,
            Value::Float64(_) => ValueIdent::Float64,
            Value::String(_) => ValueIdent::String,
            Value::Char(_) => ValueIdent::Char,
            Value::Bytes(_) => ValueIdent::Bytes,
        }
    }

    fn body(&self) -> Vec<u8> {
        match &self {
            Value::Boolean(boolean) => vec![if *boolean { 0x01 } else { 0x00 }],
            Value::Unsigned8(value) => value.to_be_bytes().to_vec(),
            Value::Unsigned16(value) => value.to_be_bytes().to_vec(),
            Value::Unsigned32(value) => value.to_be_bytes().to_vec(),
            Value::Unsigned64(value) => value.to_be_bytes().to_vec(),
            Value::Unsigned128(value) => value.to_be_bytes().to_vec(),
            Value::Signed8(value) => value.to_be_bytes().to_vec(),
            Value::Signed16(value) => value.to_be_bytes().to_vec(),
            Value::Signed32(value) => value.to_be_bytes().to_vec(),
            Value::Signed64(value) => value.to_be_bytes().to_vec(),
            Value::Signed128(value) => value.to_be_bytes().to_vec(),
            Value::Float32(value) => value.to_be_bytes().to_vec(),
            Value::Float64(value) => value.to_be_bytes().to_vec(),
            Value::String(string) => encode_string(string),
            Value::Char(c) => encode_term_string(&c.to_string()),
            Value::Bytes(array) => join_bytes(
                encode_variable_length(array.len() as u32), &array)
        }
    }
}