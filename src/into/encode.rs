use crate::{Element, Number, Value};

macro_rules! number_from {
    ($id: ident, $type: ty) => {
        impl From<$type> for Number {
            fn from(value: $type) -> Self {
                Number::$id(value)
            }
        }
        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Value::Number(Number::$id(value))
            }
        }
        impl From<$type> for Element {
            fn from(value: $type) -> Self {
                Element::Value(Value::Number(Number::$id(value)))
            }
        }
    };
}

number_from!(Unsigned8, u8);
number_from!(Signed8, i8);
number_from!(Unsigned16, u16);
number_from!(Signed16, i16);
number_from!(Unsigned32, u32);
number_from!(Signed32, i32);
number_from!(Unsigned64, u64);
number_from!(Signed64, i64);
number_from!(Unsigned128, u128);
number_from!(Signed128, i128);
number_from!(Float32, f32);
number_from!(Float64, f64);

macro_rules! value_from {
    ($id: ident, $type: ty) => {
        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Value::$id(value)
            }
        }
        impl From<$type> for Element {
            fn from(value: $type) -> Self {
                Element::Value(Value::$id(value))
            }
        }
    };
}

macro_rules! value_from_proc {
    ($id: ident, $type: ty, $func: expr) => {
        impl From<$type> for Value {
            fn from(value: $type) -> Self {
                Value::$id($func(value))
            }
        }
        impl From<$type> for Element {
            fn from(value: $type) -> Self {
                Element::Value(Value::$id($func(value)))
            }
        }
    };
}

value_from!(Boolean, bool);
value_from!(Char, char);
value_from!(String, String);
value_from!(Bytes, Vec<u8>);
value_from!(UUID, uuid::Uuid);

value_from_proc!(String, &str, String::from);
value_from_proc!(Bytes, &[u8], Vec::from);