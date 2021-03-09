pub enum ElementIdent {
    Unit,
    Value,
    Option,
    Struct,
    List,
    Map,
    Array
}

pub enum ValueIdent {
    Boolean,
    Number(NumberIdent),
    String,
    Char,
    Bytes,
    Uuid
}

pub enum NumberIdent {
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Unsigned128,

    Signed8,
    Signed16,
    Signed32,
    Signed64,
    Signed128,

    Float32,
    Float64
}

pub trait Ident {
    type IdentType;
    fn ident(&self) -> Self::IdentType;
}