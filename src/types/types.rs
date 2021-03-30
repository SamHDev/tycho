use std::collections::HashMap;

use crate::types::ident::ValueIdent;

#[derive(Debug, Clone, PartialOrd)]
/// A numerical value tag, used when creating tycho data.
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq)]
/// A primitive terminating type, used when creating tycho data.
///
/// Values can contains numerical values using [numbers](crate::Number)
pub enum Value {
    /// A null value, representing no data.
    ///
    /// Used as a placeholder type.
    /// When handling no data, use a unit instead.
    Null,

    /// A boolean value representing `true` or `false`.
    Boolean(bool),

    /// A UTF-8 length prefixed string.
    String(String),

    /// A single UTF-8 character, (not length or terminated)
    Char(char),

    /// A numerical value.
    Number(Number),

    /// A length prefixed set of unsigned octet bytes.
    Bytes(Vec<u8>),

    /// A 16 bytes unique identifier
    UUID(uuid::Uuid),
}

#[derive(Debug, Clone, PartialEq)]
/// A element tag, used to build tycho data structures.
///
/// Elements represent zero, one or multiple pieces of data.
/// They are non-primitive non-terminating and have the ability to contain typed and
/// untyped values or elements.
pub enum Element {
    /// ### Unit
    /// A element representing no data.
    ///
    /// Units are used when serializing from the following types:
    /// - Unit Struct (`struct Foo;`)
    /// - Unit Variant (`enum Foo { Bar, Baz }`)
    /// - Unit (`()`)
    Unit,

    /// ### Value
    /// A element representing a primitive value.
    ///
    /// A value element contains a [`Value`](crate::Value) object as its payload.
    /// Data structures that only contain a value with be prefixed with the ValueIdent.
    ///
    /// A value can represent any of the following types:
    /// - `bool`
    /// - `&str`,
    /// - `String`
    /// - `char`
    /// - `number` - `u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64`
    /// - `bytes` - `Vec<u8>`
    Value(Value),

    /// ### Option
    /// An element representing a Some or None value.
    ///
    /// This element maps directly to Rust `Option<T>` type,
    /// and can have a payload of an element when some.
    Option(Option<Box<Element>>),

    /// ### Variant
    /// An element representing a named variable type.
    ///
    /// This element represents a rust enum.
    ///
    /// #### Unit Variants
    /// e.g. `enum Example { Foo, Bar, Baz }`
    ///
    /// A unit variant is a enum with no data.
    ///
    /// Within tycho, the value `Example::Foo` would be represented as `Variant("Foo", Unit)`
    ///
    /// #### Value Variants
    /// e.g. `enum Example { Foo(String), Bar(bool), Baz(u8) }`
    ///
    /// A 'new type' enum containing a single piece of data.
    ///
    /// Within tycho, the value `Example::Bar(false)` would be represented as `Variant("Bar", Value(Bool(0)))`
    ///
    /// #### Tuple Variants
    /// e.g. `enum Example { Foo(u8, bool), Bar(u16, bool), Baz(u32, bool) }`
    ///
    /// A 'tuple' enum containing a multiple unnamed pieces of data.
    ///
    /// Within tycho, the value `Example::Baz(10, true)` would be represented as:
    /// `Variant("Baz, List([Value(Unsigned32(10)), Value(Bool(1)]))`
    ///
    /// #### Struct Variants
    /// e.g. `enum Example { Foo { bar: char } }`
    ///
    /// A 'tuple' enum containing a multiple unnamed pieces of data.
    ///
    /// Within tycho, the value `Example::Foo { bar: 'P'}` would be represented as:
    /// `Variant("Baz, Struct({"bar": Value(Char('P'))))`
    ///
    Variant(String, Box<Element>),

    /// ### Struct
    /// An element representing an untyped string key - element paring.
    ///
    /// Directly maps to rusts `struct` object within the serialisation process.
    Struct(HashMap<String, Element>),

    /// ### List
    /// An element representing an untyped collection of elements.
    ///
    /// Directly maps to any tuple or heterogeneous array.
    List(Vec<Element>),

    /// ### Array
    /// An element representing a strictly typed collection of values.
    ///
    /// Unlike lists, arrays can only store terminating values and homogeneous.
    Array(ValueIdent, Vec<Value>),

    /// ### Map
    /// An element representing a strictly typed key - to untyped element pair.
    ///
    /// They key of a map can be any terminating value, and is strictly typed.
    /// The value of a map can be any element and is homogeneous.
    Map(ValueIdent, HashMap<Value, Element>),

    /// ### Compression Marker
    /// Contains an element that will be g-zip compressed.
    #[cfg(feature="compression")]
    Compression(Box<Element>),
    #[cfg(not(feature="compression"))]
    Compression(Vec<u8>),
}
