use std::collections::HashMap;

use crate::types::ident::ValueIdent;

#[derive(Debug, Clone, PartialOrd)]
pub enum Number {
    Bit(bool),
    Unsigned8(u8),
    Signed8(i8),
    Unsigned16(u16),
    Signed16(i16),
    Unsigned32(u32),
    Signed32(i32),
    Unsigned64(u64),
    Signed64(i64),
    Unsigned128(u128),
    Signed128(i128),
    Float32(f32),
    Float64(f64)
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Char(char),
    Number(Number),
    Bytes(Vec<u8>),
    UUID(uuid::Uuid),
}

#[derive(Debug, Clone)]
pub enum Element {
    Unit,
    Value(Value),
    Option(Option<Box<Element>>),
    Variant(String, Box<Element>),
    Struct(HashMap<String, Element>),
    List(Vec<Element>),
    Array(ValueIdent, Vec<Value>),
    Map(ValueIdent, HashMap<Value, Element>),
    Compression(Box<Element>)
}
