use crate::{Element, Number, Value};

impl From<Value> for Element {
    fn from(v: Value) -> Self {
        Self::Value(v)
    }
}

impl From<Number> for Value {
    fn from(v: Number) -> Self {
        Self::Number(v)
    }
}
