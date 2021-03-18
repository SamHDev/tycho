use std::fmt;

use crate::{Element, Number, Value};

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Number::Bit(x) => fmt::Display::fmt(x, f),
            Number::Unsigned8(x) => fmt::Display::fmt(x, f),
            Number::Signed8(x) => fmt::Display::fmt(x, f),
            Number::Unsigned16(x) => fmt::Display::fmt(x, f),
            Number::Signed16(x) => fmt::Display::fmt(x, f),
            Number::Unsigned32(x) => fmt::Display::fmt(x, f),
            Number::Signed32(x) => fmt::Display::fmt(x, f),
            Number::Unsigned64(x) => fmt::Display::fmt(x, f),
            Number::Signed64(x) => fmt::Display::fmt(x, f),
            Number::Unsigned128(x) => fmt::Display::fmt(x, f),
            Number::Signed128(x) => fmt::Display::fmt(x, f),
            Number::Float32(x) => fmt::Display::fmt(x, f),
            Number::Float64(x) => fmt::Display::fmt(x, f),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Null => f.write_str("null"),
            Value::Boolean(x) => fmt::Display::fmt(x, f),
            Value::String(x) => fmt::Display::fmt(x, f),
            Value::Char(x) => fmt::Display::fmt(x, f),
            Value::Number(x) => fmt::Display::fmt(x, f),
            Value::Bytes(x) => fmt::Debug::fmt(x, f),
            Value::UUID(x) => fmt::Display::fmt(x, f),
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Element::Unit => fmt::Debug::fmt(&(), f),
            Element::Value(x) => fmt::Display::fmt(x, f),
            Element::Option(x) => match x {
                Some(x) => {
                    f.write_str("Some(")?;
                    fmt::Display::fmt(x, f)?;
                    f.write_str(")")
                },
                None => f.write_str("None")
            }
            Element::Variant(n, x) => {
                f.write_str("\"")?;
                f.write_str(&n)?;
                f.write_str("\" { ")?;
                fmt::Display::fmt(x, f)?;
                f.write_str(" }")
            }
            Element::Struct(x) => fmt::Debug::fmt(x, f),
            Element::List(x) => fmt::Debug::fmt(x, f),
            Element::Array(_, x) => fmt::Debug::fmt(x, f),
            Element::Map(_, x) => fmt::Debug::fmt(x, f),
            Element::Compression(x) => fmt::Display::fmt(x, f),
        }
    }
}