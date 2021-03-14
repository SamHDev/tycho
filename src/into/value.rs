use crate::{Value, ValueIdent};

pub trait ValueType: Into<Value> {
    const IDENT: ValueIdent;

}

macro_rules! impl_value_type {
    ($type: ty, $ident: ident) => {
         impl ValueType for $type {
            const IDENT: ValueIdent = $ident;
         }
    };
}

impl_value_type!(bool, ValueIdent::Bool);
impl_value_type!(bool, ValueIdent::Bool);