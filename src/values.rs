use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq)]
/// A primitive, terminating value.
pub enum Value {
    Boolean(bool),

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
    Float64(f64),

    String(String), // Length String 0x02 0x69 0x42
    Char(char),

    Bytes(Vec<u8>),
}


impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            Value::Boolean(value) => Hash::hash(value, state),
            Value::Unsigned8(value) => Hash::hash(value, state),
            Value::Unsigned16(value) => Hash::hash(value, state),
            Value::Unsigned32(value) => Hash::hash(value, state),
            Value::Unsigned64(value) => Hash::hash(value, state),
            Value::Unsigned128(value) => Hash::hash(value, state),
            Value::Signed8(value) => Hash::hash(value, state),
            Value::Signed16(value) => Hash::hash(value, state),
            Value::Signed32(value) => Hash::hash(value, state),
            Value::Signed64(value) => Hash::hash(value, state),
            Value::Signed128(value) => Hash::hash(value, state),
            Value::Float32(_) => {panic!("Error! Floating point numbers are un-hashable")},
            Value::Float64(_) => {panic!("Error! Floating point numbers are un-hashable")},
            Value::String(value) => Hash::hash(value, state),
            Value::Char(value) => Hash::hash(value, state),
            Value::Bytes(value) => Hash::hash(value, state),
        }
    }
}
impl Eq for Value {}

pub(crate) trait ValueCanHash {
    fn can_hash(&self) -> bool;
}

impl ValueCanHash for Value {
    fn can_hash(&self) -> bool {
        match self {
            Value::Float64(_) => false,
            Value::Float32(_) => false,
            _ => true
        }
    }
}
