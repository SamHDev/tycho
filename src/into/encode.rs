use crate::{Number, Value};

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

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}

impl From<char> for Value {
    fn from(value: char) -> Self {
        Value::Char(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Vec<u8>> for Value {
    fn from(value: Vec<u8>) -> Self {
        Value::Bytes(value)
    }
}

impl From<uuid::Uuid> for Value {
    fn from(value: uuid::Uuid) -> Self {
        Value::UUID(value)
    }
}