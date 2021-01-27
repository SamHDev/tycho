use crate::{Value, Element};

pub trait ValueUtil {
    fn bool(v: bool) -> Self;
    fn u8(v: u8) -> Self;
    fn u16(v: u16) -> Self;
    fn u32(v: u32) -> Self;
    fn u64(v: u64) -> Self;
    fn u128(v: u128) -> Self;
    fn i8(v: i8) -> Self;
    fn i16(v: i16) -> Self;
    fn i32(v: i32) -> Self;
    fn i64(v: i64) -> Self;
    fn i128(v: i128) -> Self;
    fn f32(v: f32) -> Self;
    fn f64(v: f64) -> Self;
    fn string(v: &str) -> Self;
    fn char(v: char) -> Self;
    //fn tstring(v: &str) -> Self;
    fn bytes(v: &[u8]) -> Self;
}

impl ValueUtil for Value {
    fn bool(v: bool) -> Self { Value::Boolean(v) }
    fn u8(v: u8) -> Self { Value::Unsigned8(v) }
    fn u16(v: u16) -> Self { Value::Unsigned16(v) }
    fn u32(v: u32) -> Self { Value::Unsigned32(v) }
    fn u64(v: u64) -> Self { Value::Unsigned64(v) }
    fn u128(v: u128) -> Self { Value::Unsigned128(v) }
    fn i8(v: i8) -> Self { Value::Signed8(v) }
    fn i16(v: i16) -> Self { Value::Signed16(v) }
    fn i32(v: i32) -> Self { Value::Signed32(v) }
    fn i64(v: i64) -> Self { Value::Signed64(v) }
    fn i128(v: i128) -> Self { Value::Signed128(v) }
    fn f32(v: f32) -> Self { Value::Float32(v) }
    fn f64(v: f64) -> Self { Value::Float64(v) }
    fn string(v: &str) -> Self { Value::String(v.to_string()) }
    fn char(v: char) -> Self { Value::Char(v) }
    //fn tstring(v: &str) -> Self { Value::TermString(v.to_string()) }
    fn bytes(v: &[u8]) -> Self { Value::Bytes(v.to_vec()) }
}

impl Into<Value> for bool { fn into(self) -> Value { Value::bool(self) } }
impl Into<Value> for u8 { fn into(self) -> Value { Value::u8(self) } }
impl Into<Value> for u16 { fn into(self) -> Value { Value::u16(self) } }
impl Into<Value> for u32 { fn into(self) -> Value { Value::u32(self) } }
impl Into<Value> for u64 { fn into(self) -> Value { Value::u64(self) } }
impl Into<Value> for u128 { fn into(self) -> Value { Value::u128(self) } }
impl Into<Value> for i8 { fn into(self) -> Value { Value::i8(self) } }
impl Into<Value> for i16 { fn into(self) -> Value { Value::i16(self) } }
impl Into<Value> for i32 { fn into(self) -> Value { Value::i32(self) } }
impl Into<Value> for i64 { fn into(self) -> Value { Value::i64(self) } }
impl Into<Value> for i128 { fn into(self) -> Value { Value::i128(self) } }
impl Into<Value> for f32 { fn into(self) -> Value { Value::f32(self) } }
impl Into<Value> for f64 { fn into(self) -> Value { Value::f64(self) } }
impl Into<Value> for String { fn into(self) -> Value { Value::string(&self) } }
impl Into<Value> for &str { fn into(self) -> Value { Value::string(self) } }
impl Into<Value> for char { fn into(self) -> Value { Value::char(self) }}

impl ValueUtil for Element {
    fn bool(v: bool) -> Self { Element::Value(Value::Boolean(v)) }
    fn u8(v: u8) -> Self { Element::Value(Value::Unsigned8(v)) }
    fn u16(v: u16) -> Self { Element::Value(Value::Unsigned16(v)) }
    fn u32(v: u32) -> Self { Element::Value(Value::Unsigned32(v)) }
    fn u64(v: u64) -> Self { Element::Value(Value::Unsigned64(v)) }
    fn u128(v: u128) -> Self { Element::Value(Value::Unsigned128(v)) }
    fn i8(v: i8) -> Self { Element::Value(Value::Signed8(v)) }
    fn i16(v: i16) -> Self { Element::Value(Value::Signed16(v)) }
    fn i32(v: i32) -> Self { Element::Value(Value::Signed32(v)) }
    fn i64(v: i64) -> Self { Element::Value(Value::Signed64(v)) }
    fn i128(v: i128) -> Self { Element::Value(Value::Signed128(v)) }
    fn f32(v: f32) -> Self { Element::Value(Value::Float32(v)) }
    fn f64(v: f64) -> Self { Element::Value(Value::Float64(v)) }
    fn string(v: &str) -> Self { Element::Value(Value::String(v.to_string())) }
    fn char(v: char) -> Self { Element::Value(Value::Char(v)) }
    //fn tstring(v: &str) -> Self { Element::Value(Value::TermString(v.to_string())) }
    fn bytes(v: &[u8]) -> Self { Element::Value(Value::Bytes(v.to_vec())) }
}

impl Into<Element> for bool { fn into(self) -> Element { Element::bool(self) } }
impl Into<Element> for u8 { fn into(self) -> Element { Element::u8(self) } }
impl Into<Element> for u16 { fn into(self) -> Element { Element::u16(self) } }
impl Into<Element> for u32 { fn into(self) -> Element { Element::u32(self) } }
impl Into<Element> for u64 { fn into(self) -> Element { Element::u64(self) } }
impl Into<Element> for u128 { fn into(self) -> Element { Element::u128(self) } }
impl Into<Element> for i8 { fn into(self) -> Element { Element::i8(self) } }
impl Into<Element> for i16 { fn into(self) -> Element { Element::i16(self) } }
impl Into<Element> for i32 { fn into(self) -> Element { Element::i32(self) } }
impl Into<Element> for i64 { fn into(self) -> Element { Element::i64(self) } }
impl Into<Element> for i128 { fn into(self) -> Element { Element::i128(self) } }
impl Into<Element> for f32 { fn into(self) -> Element { Element::f32(self) } }
impl Into<Element> for f64 { fn into(self) -> Element { Element::f64(self) } }
impl Into<Element> for String { fn into(self) -> Element { Element::string(&self) } }
impl Into<Element> for &str { fn into(self) -> Element { Element::string(self) } }
impl Into<Element> for char { fn into(self) -> Element { Element::char(self) }}

impl Into<Element> for Value { fn into(self) -> Element { Element::Value(self) }}