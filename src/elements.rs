use crate::Value;
use std::collections::HashMap;

/// A non-terminating value, structure or array.
#[derive(Clone, Debug)]
pub enum Element {
    /// An anonymous value containing no data.
    Unit,
    /// A primitive, terminating value.
    Value(Value),
    /// A Some/None Value
    Option(Option<Box<Element>>),

    // Variant/Structure Types
    /// A variable type sequence of elements.
    Array(Vec<Element>),
    /// A variable type key-element map.
    Struct(HashMap<String, Element>),
    /// A variable element type with a given name.
    Variant(String, Box<Element>),

    /// A statically-typed value-element map.
    Map(HashMap<Value, Element>),
    /// A statically-typed sequence of elements.
    List(Vec<Value>)

}