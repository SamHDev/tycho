use std::hash::{Hash, Hasher};

use crate::{Number, Value};

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            Value::Null => 0.hash(state),
            Value::Boolean(b) => b.hash(state),
            Value::String(s) => s.hash(state),
            Value::Char(c) => c.hash(state),
            Value::Number(n) => n.hash(state),
            Value::Bytes(b) => b.hash(state),
            Value::UUID(u) => u.hash(state)
        }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            Number::Bit(v) => v.hash(state),
            Number::Unsigned8(n) => n.hash(state),
            Number::Signed8(n) => n.hash(state),
            Number::Unsigned16(n) => n.hash(state),
            Number::Signed16(n) => n.hash(state),
            Number::Unsigned32(n) => n.hash(state),
            Number::Signed32(n) => n.hash(state),
            Number::Unsigned64(n) => n.hash(state),
            Number::Signed64(n) => n.hash(state),
            Number::Unsigned128(n) => n.hash(state),
            Number::Signed128(n) => n.hash(state),
            Number::Float32(n) => (*n as u32).hash(state),
            Number::Float64(n) => (*n as u64).hash(state),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        if let (Number::Bit(a), Number::Bit(b)) = (&self, &other) { a == b }
        else if let (Number::Unsigned8(a), Number::Unsigned8(b)) = (&self, &other) { a == b }
        else if let (Number::Signed8(a), Number::Signed8(b)) = (&self, &other) { a == b }
        else if let (Number::Unsigned16(a), Number::Unsigned16(b)) = (&self, &other) { a == b }
        else if let (Number::Signed16(a), Number::Signed16(b)) = (&self, &other) { a == b }
        else if let (Number::Unsigned32(a), Number::Unsigned32(b)) = (&self, &other) { a == b }
        else if let (Number::Signed32(a), Number::Signed32(b)) = (&self, &other) { a == b }
        else if let (Number::Unsigned64(a), Number::Unsigned64(b)) = (&self, &other) { a == b }
        else if let (Number::Signed64(a), Number::Signed64(b)) = (&self, &other) { a == b }
        else if let (Number::Unsigned128(a), Number::Unsigned128(b)) = (&self, &other) { a == b }
        else if let (Number::Signed128(a), Number::Signed128(b)) = (&self, &other) { a == b }
        else if let (Number::Float32(a), Number::Float32(b)) = (&self, &other) { a == b }
        else if let (Number::Float64(a), Number::Float64(b)) = (&self, &other) { a == b }
        else { false }
    }
}
impl Eq for Number {}