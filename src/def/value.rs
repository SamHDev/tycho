use uuid::Uuid;
use crate::def::ident::{Ident, ValueIdent, NumberIdent};

pub enum Value {
    Boolean(bool),
    Number(Number),
    String(String),
    Char(char),
    Bytes(Vec<u8>),
    Uuid(Uuid),
}

pub enum Number {
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Unsigned64(u64),
    Unsigned128(u128),

    Signed8(i8),
    Signed16(i16),
    Signed32(i32),
    Signed64(i64),
    Signed128(i128),

    Float32(f32),
    Float64(f64)
}

impl Ident for Value {
    type IdentType = ValueIdent;

    fn ident(&self) -> Self::IdentType {
        match &self {
            Value::Boolean(_) => ValueIdent::Boolean,
            Value::Number(_) => ValueIdent::Number,
            Value::String(_) => ValueIdent::String,
            Value::Char(_) => ValueIdent::Char,
            Value::Bytes(_) => ValueIdent::Bytes,
            Value::Uuid(_) => ValueIdent::Uuid,
        }
    }
}

impl Ident for Number {
    type IdentType = NumberIdent;

    fn ident(&self) -> Self::IdentType {
        match &self {
            Number::Unsigned8(_) => NumberIdent::Unsigned8,
            Number::Unsigned16(_) => NumberIdent::Unsigned16,
            Number::Unsigned32(_) => NumberIdent::Unsigned32,
            Number::Unsigned64(_) => NumberIdent::Unsigned64,
            Number::Unsigned128(_) => NumberIdent::Unsigned128,
            Number::Signed8(_) => NumberIdent::Signed8,
            Number::Signed16(_) => NumberIdent::Signed16,
            Number::Signed32(_) => NumberIdent::Signed32,
            Number::Signed64(_) => NumberIdent::Signed64,
            Number::Signed128(_) => NumberIdent::Signed128,
            Number::Float32(_) => NumberIdent::Float32,
            Number::Float64(_) => NumberIdent::Float64,
        }
    }
}