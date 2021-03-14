use crate::{Element, Value, Number};

// Inter-op
impl<T: Into<Value>> Into<Element> for T {
    fn into(self) -> Element {
        Element::Value(self.into())
    }
}
impl<T: Into<Number>> Into<Value> for T {
    fn into(self) -> Value {
        Value::Number(self.into())
    }
}

// Numerical values
impl Into<Number> for u8 {
    fn into(self) -> Number {
        Number::Unsigned8(self)
    }
}
impl Into<Number> for u16 {
    fn into(self) -> Number {
        Number::Unsigned16(self)
    }
}
impl Into<Number> for u32 {
    fn into(self) -> Number {
        Number::Unsigned32(self)
    }
}
impl Into<Number> for u64 {
    fn into(self) -> Number {
        Number::Unsigned64(self)
    }
}
impl Into<Number> for u128 {
    fn into(self) -> Number {
        Number::Unsigned128(self)
    }
}
impl Into<Number> for i8 {
    fn into(self) -> Number {
        Number::Signed8(self)
    }
}
impl Into<Number> for i16 {
    fn into(self) -> Number {
        Number::Signed16(self)
    }
}
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number::Signed32(self)
    }
}
impl Into<Number> for i64 {
    fn into(self) -> Number {
        Number::Signed64(self)
    }
}
impl Into<Number> for i128 {
    fn into(self) -> Number {
        Number::Signed128(self)
    }
}
impl Into<Number> for f32 {
    fn into(self) -> Number {
        Number::Float32(self)
    }
}
impl Into<Number> for f64 {
    fn into(self) -> Number {
        Number::Float64(self)
    }
}

// Values
impl Into<Value> for bool {
    fn into(self) -> Value {
        Value::Boolean(self)
    }
}
impl Into<Value> for char {
    fn into(self) -> Value {
        Value::Char(self)
    }
}
impl Into<Value> for String {
    fn into(self) -> Value {
        Value::String(self)
    }
}
impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::String(self.to_string())
    }
}
impl Into<Value> for Vec<u8> {
    fn into(self) -> Value {
        Value::String(self)
    }
}