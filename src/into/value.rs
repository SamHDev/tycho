use crate::Value;
use crate::types::ident::{NumberIdent, ValueIdent};

pub trait ValueType: Into<Value> {
    const IDENT: ValueIdent;
}

macro_rules! impl_value_type {
    ($type: ty, $ident: expr) => {
         impl ValueType for $type {
            const IDENT: ValueIdent = $ident;
         }
    };
}

impl_value_type!(bool, ValueIdent::Boolean);
impl_value_type!(u8, ValueIdent::Number(NumberIdent::Unsigned8));
impl_value_type!(i8, ValueIdent::Number(NumberIdent::Signed8));
impl_value_type!(u16, ValueIdent::Number(NumberIdent::Unsigned16));
impl_value_type!(i16, ValueIdent::Number(NumberIdent::Signed16));
impl_value_type!(u32, ValueIdent::Number(NumberIdent::Unsigned32));
impl_value_type!(i32, ValueIdent::Number(NumberIdent::Signed32));
impl_value_type!(u64, ValueIdent::Number(NumberIdent::Unsigned64));
impl_value_type!(i64, ValueIdent::Number(NumberIdent::Signed64));
impl_value_type!(u128, ValueIdent::Number(NumberIdent::Unsigned128));
impl_value_type!(i128, ValueIdent::Number(NumberIdent::Signed128));
impl_value_type!(f32, ValueIdent::Number(NumberIdent::Float32));
impl_value_type!(f64, ValueIdent::Number(NumberIdent::Float64));
impl_value_type!(String, ValueIdent::String);
impl_value_type!(&str, ValueIdent::String);
impl_value_type!(char, ValueIdent::Char);
//impl_value_type!((), ValueIdent::Null);
impl_value_type!(Vec<u8>, ValueIdent::Bytes);
impl_value_type!(uuid::Uuid, ValueIdent::UUID);