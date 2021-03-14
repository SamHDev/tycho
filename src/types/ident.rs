//! Type prefixes/identities used within the marshall and unmarshall processes - Returned in errors.

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum NumberIdent {
    Bit,
    Unsigned8,
    Signed8,
    Unsigned16,
    Signed16,
    Unsigned32,
    Signed32,
    Unsigned64,
    Signed64,
    Unsigned128,
    Signed128,
    Float32,
    Float64
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ValueIdent {
    Null,
    Boolean,
    String,
    Char,
    Number(NumberIdent),
    Bytes,
    UUID
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ElementIdent {
    Unit,
    Value,
    Some,
    None,
    Variant,
    Struct,
    List,
    Array,
    Map,
    Compression
}