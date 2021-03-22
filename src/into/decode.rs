use std::convert::TryFrom;

use crate::{Element, Number, Value};

macro_rules! number_to {
    ($id: ident, $type: ty) => {
        impl TryFrom<Number> for $type {
            type Error = ();

            fn try_from(value: Number) -> Result<Self, Self::Error> {
                if let Number::$id(x) = value { return Ok(x) } else { Err(()) }
            }
        }
        impl TryFrom<Value> for $type {
            type Error = ();

            fn try_from(value: Value) -> Result<Self, Self::Error> {
                if let Value::Number(Number::$id(x)) = value { return Ok(x) } else { Err(()) }
            }
        }
        impl TryFrom<Element> for $type {
            type Error = ();

            fn try_from(value: Element) -> Result<Self, Self::Error> {
                if let Element::Value(Value::Number(Number::$id(x))) = value { return Ok(x) }
                 else { Err(()) }
            }
        }
    };
}

number_to!(Unsigned8, u8);
number_to!(Signed8, i8);
number_to!(Unsigned16, u16);
number_to!(Signed16, i16);
number_to!(Unsigned32, u32);
number_to!(Signed32, i32);
number_to!(Unsigned64, u64);
number_to!(Signed64, i64);
number_to!(Unsigned128, u128);
number_to!(Signed128, i128);
number_to!(Float32, f32);
number_to!(Float64, f64);


macro_rules! value_to {
    ($id: ident, $type: ty) => {
        impl TryFrom<Value> for $type {
            type Error = ();
            fn try_from(value: Value) -> Result<Self, Self::Error> {
                if let Value::$id(x) = value { return Ok(x) } else { Err(()) }
            }
        }
        impl TryFrom<Element> for $type {
            type Error = ();
            fn try_from(value: Element) -> Result<Self, Self::Error> {
                if let Element::Value(Value::$id(x)) = value { return Ok(x) } else { Err(()) }
            }
        }
    };
}

/*
macro_rules! value_to_proc {
    ($id: ident, $type: ty, $func: expr) => {
       impl TryFrom<Value> for $type {
            type Error = ();
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                if let Value::$id(x) = value { return Ok($func(x)) } else { Err(()) }
            }
        }
        impl TryFrom<Element> for $type {
            type Error = ();
            fn try_from(value: $type) -> Result<Self, Self::Error> {
                if let Element::Value(Value::$id(x)) = value { return Ok($func(x)) } else { Err(()) }
            }
        }
    };
}*/

value_to!(Boolean, bool);
value_to!(Char, char);
value_to!(String, String);
value_to!(Bytes, Vec<u8>);
value_to!(UUID, uuid::Uuid);
