use crate::{Element, ElementIdent, Value, ValueIdent, Number, NumberIdent};

pub trait Ident {
    type IdentType;
    fn ident(&self) -> Self::IdentType;
}

impl Ident for Element {
    type IdentType = ElementIdent;

    fn ident(&self) -> Self::IdentType {
        match &self {
            Element::Unit => ElementIdent::Unit,
            Element::Value(_) => ElementIdent::Value,
            Element::Option(x) => match x {
                Some(_) =>  ElementIdent::Some,
                None =>  ElementIdent::None,
            }
            Element::Variant(_, _) =>  ElementIdent::Variant,
            Element::Struct(_) =>  ElementIdent::Struct,
            Element::List(_) =>  ElementIdent::List,
            Element::Array(_, _) =>  ElementIdent::Array,
            Element::Map(_, _) =>  ElementIdent::Map,
            Element::Compression(_) =>  ElementIdent::Compression
        }
    }
}

impl Ident for Value {
    type IdentType = ValueIdent;

    fn ident(&self) -> Self::IdentType {
        match &self {
            Value::Null => ValueIdent::Null,
            Value::Boolean(_) => ValueIdent::Boolean,
            Value::String(_) => ValueIdent::String,
            Value::Char(_) => ValueIdent::Char,
            Value::Number(x) => ValueIdent::Number(x.ident()),
            Value::Bytes(_) => ValueIdent::Bytes,
            Value::UUID(_) => ValueIdent::UUID
        }
    }
}

impl Ident for Number {
    type IdentType = NumberIdent;

    fn ident(&self) -> Self::IdentType {
        match &self {
            Number::Bit(_) => NumberIdent::Bit,
            Number::Unsigned8(_) => NumberIdent::Unsigned8,
            Number::Signed8(_) => NumberIdent::Signed8,
            Number::Unsigned16(_) => NumberIdent::Unsigned16,
            Number::Signed16(_) => NumberIdent::Signed16,
            Number::Unsigned32(_) => NumberIdent::Unsigned32,
            Number::Signed32(_) => NumberIdent::Signed32,
            Number::Unsigned64(_) => NumberIdent::Unsigned64,
            Number::Signed64(_) => NumberIdent::Signed64,
            Number::Unsigned128(_) => NumberIdent::Unsigned128,
            Number::Signed128(_) => NumberIdent::Signed128,
            Number::Float32(_) => NumberIdent::Float32,
            Number::Float64(_) => NumberIdent::Float64,
        }
    }
}