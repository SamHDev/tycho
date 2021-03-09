use crate::def::value::Value;
use std::collections::HashMap;
use crate::def::ident::{Ident, ElementIdent};

pub enum Element {
    Unit,
    Value(Value),

    Option(Option<Box<Element>>),

    Struct(HashMap<String, Element>),
    List(Vec<Element>),

    Map(HashMap<Value, Element>),
    Array(Vec<Value>)
}

impl Ident for Element {
    type IdentType = ElementIdent;

    fn ident(&self) -> Self::IdentType {
        match &self {
            Element::Unit => ElementIdent::Unit,
            Element::Value(_) => ElementIdent::Value,
            Element::Option(_) => ElementIdent::Option,
            Element::Struct(_) => ElementIdent::Struct,
            Element::List(_) => ElementIdent::List,
            Element::Map(_) => ElementIdent::Map,
            Element::Array(_) => ElementIdent::Array
        }
    }
}