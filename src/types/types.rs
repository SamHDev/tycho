pub enum Number {
    Bit(bool),
    Unsigned8(u8),
    Signed8(i8),
    Unsigned16(u16),
    Signed16(i16),
    Unsigned32(u32),
    Signed32(i32),
    Unsigned64(u64),
    Signed64(i64),
    Unsigned128(u128),
    Signed128(i128),
    Float32(f32),
    Float64(f64)
}

pub enum Value {
    Null,
    Boolean(bool),
    String(String),
    Char(char),
    Number(Value),
    Bytes(Vec<u8>),
    UUID(uuid::Uuid)
}

pub enum Element {
    Unit,
    Value,
    Option(Option<Box<Element>>),
    Variant(String, Box<Element>),
    Struct(),
    List,
    Array,
    Map,
    Compression
}