//! # Tycho
//! A self describing binary format designed around the serde data model.
//!
//! ## Getting started
//! To get started, you can import the latest release from [crates.io](https://crates.io/crates/tycho)
//! ```toml
//! [dependencies]
//! tycho = "0.0.5"
//! ```
//! **or** you can use the developmental version from the public [git repository](https://github.com/SamHDev/tycho)
//! ```toml
//! [dependencies.tycho]
//! git = "https://github.com/SamHDev/tycho.git"
//! ```
//!
//! ## Tycho
//! Here is the datatypes within the Tycho format:
//! - `Unit` - A value containing no data. Maps directly to `()`
//! - `Value` - A primitive, terminating type.
//!     - `Boolean` - true or false value (`bool`)
//!     - `Unsigned8` - 8-bit unsigned integer (`u8`)
//!     - `Unsigned16` - 16-bit unsigned integer (`u16`)
//!     - `Unsigned32` - 32-bit unsigned integer (`u32`)
//!     - `Unsigned64` - 64-bit unsigned integer (`u64`)
//!     - `Unsigned128` - 128-bit unsigned integer (`u128`)
//!     - `Signed8` - 8-bit two compelement signed integer (`i8`)
//!     - `Signed16` - 16-bit two compelement signed integer (`i16`)
//!     - `Signed32` - 32-bit two compelement signed integer (`i32`)
//!     - `Signed64` - 64-bit two compelement signed integer (`i64`)
//!     - `Signed128` - 128-bit two compelement signed integer (`i128`)
//!     - `String` - An UTF-8 string (`&str`)
//!     - `Char` - A single UTF-8 character. (`char`)
//!     - `Bytes` - An array of 8-bit bytes. (`&[u8]`)
//! - `Option` - A Some or None value (`Option<T>`)
//! - `Array` - An untyped array of elements
//! - `Structure` - A untyped string-value map (`struct`)
//! - `Map` - A typed key-value map (`Map<K, V>`)
//! - `List` - A typed array of values (`Vec<T>`)
//! - `Variant` - A variable element type with a given name. (`enum`)
//!
//! #### Variants
//! Enums in rust, are arguably one of its best features.
//! Serde has four types of varaints in its data model.
//! Rather than implementing each, the `Variant` type is used.
//! ```
//! enum Enum {
//!     Foo,
//!     // Enum::Foo -> Variant("Foo", Unit)
//!
//!     Bar(String),
//!     // Enum::Bar(...)` -> Variant("bar", ...)
//!
//!     Baz(bool, bool, bool),
//!     // Enum::Baz(..., ..., ...) -> Variant("baz", List(..., ..., ...))
//!
//!     Qux { quux: String, quuz: bool }
//!     // Enum::Qux { ... } => Variant("qux", Structure(...))
//! }
//! ```
//!
//! ## Tycho Elements
//! Tycho elements can be instantiated directly rather than being serialised through serde.
//! ```
//! use tycho::{Element, Value, ValueUtil, ElementUtil};
//!
//! // Instantiation
//! let string = Element::string("Hello World!");
//! let array = Element::array(vec![string]);
//!
//! // Encoding
//! let bytes = tycho::encode(array);
//!
//! // Decoding
//! let data = tycho::decode(bytes).unwrap();
//!
//! // Unpacking
//! if let Element::Array(array) = data {
//!     if let Some(Element::Value(Value::String(string))) = array.get(0) {
//!         assert_eq!(string, "Hello World!")
//!     }
//!     # else { panic!() }
//! }
//! # else { panic!() }
//! ```
//!
//! ## Serialisation
//! The main feature of tycho is the ability to serialise and deserialise using the [serde](https://serde.rs) framework.
//! ```
//! # use serde::Serialize;
//! # use tycho;
//!
//! // Define a serilisable object
//! #[derive(Serialize)]
//! pub struct Person {
//!     name: String,
//!     foo: bool,
//!     bar: u32
//! }
//!
//! // Instantiate an structure
//! let person = Person { name: "Dirk Gently".to_string(), foo: false, bar: 42069 };
//!
//! // Serialise
//! let element = tycho::to_element(&person).unwrap();
//!
//! // Serialise and Encode
//! let bytes = tycho::to_bytes(&person).unwrap();
//! ```
//!
//!
//! ## Deserialisation
//! ```
//! # use serde::Deserialize;
//! # use tycho;
//! # use tycho::Element;
//! # use tycho::util::TychoStruct;
//!
//! // Define a deserialisable object (see above)
//! # #[derive(Deserialize)]
//! # pub struct Person {
//! #    name: String,
//! #    foo: bool,
//! #    bar: u32
//! # }
//!
//! // Decode bytes
//! let bytes = /* vec![ ... ] */
//! # vec![ 64, 3, 3, 102, 111, 111, 16, 0, 3, 98, 97, 114, 19, 0, 0, 164, 85, 4, 110, 97, 109, 101, 29, 11, 68, 105, 114, 107, 32, 71, 101, 110, 116, 108, 121 ];
//!
//! let person: Person = tycho::from_bytes(bytes).unwrap();
//!
//! // Decode element
//! let mut element = TychoStruct::new();
//! element.insert("name", "Arthur Dent");
//! element.insert("foo", false);
//! element.insert("bar", 69420u32);
//!
//! let person: Person = tycho::from_element(element.into()).unwrap();
//! ```

mod encode;
pub mod util;
mod values;
mod elements;
mod ident;
mod decode;
pub mod de;
pub mod ser;

pub use values::Value;
pub use elements::Element;

pub use ser::error::TychoSerializerError as SerializeError;
pub use de::error::TychoDeserializeError as DeserializeError;

use crate::encode::element::ElementEncoder;
use crate::decode::DecodeError;
use serde::{Serialize};
use crate::ser::ser::TychoSerializer;
use serde::de::DeserializeOwned;

pub use util::ValueUtil;
pub use util::ElementUtil;


/// Encode an element into bytes.
/// ```
/// use tycho::{Element, ValueUtil, encode};
///
/// let element = Element::string("Hello World");
/// let bytes = encode(element);
///
/// assert_eq!(bytes, vec![29, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100])
/// ```
/// This function does not create an error.
#[doc(inline)]
pub fn encode(element: Element) -> Vec<u8> {
    element.encode()
}

/// Decode an element from bytes.
/// ```
/// use tycho::{Element, Value, decode};
///
/// let element = decode(vec![29, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]);
/// if let Ok(Element::Value(Value::String(string))) = element {
///     assert_eq!(string, "Hello World")
/// }
/// # else { panic!("Invalid Value") }
/// ```
#[doc(inline)]
pub fn decode(data: Vec<u8>) -> Result<Element, DecodeError> {
    decode::decode(data)
}

/// Deserialize from bytes
/// ```
/// use tycho::{Element, Value, decode, from_bytes};
/// use serde::Deserialize;
///
/// #[derive(Deserialize, Debug, PartialEq)]
/// pub struct Example {
///     foo: String,
///     bar: u8,
///     baz: bool
/// }
///
/// let bytes = vec![64, 3, 3, 98, 97, 114, 17, 42, 3, 102, 111, 111, 29, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 3, 98, 97, 122, 16, 1];
/// let element: Example = from_bytes(bytes).unwrap();
///
/// assert_eq!(
///     element,
///     Example {
///         foo: "Hello World".to_string(),
///         bar: 42,
///         baz: true
///     }
/// )
/// ```
pub fn from_bytes<T: DeserializeOwned>(data: Vec<u8>) -> Result<T, DeserializeError> {
    match decode(data) {
        Ok(element) =>  T::deserialize(de::de::TychoDeserializer::new(element)),
        Err(e) => Err(DeserializeError::DecodeError(e))
    }
}

/// Serialize into bytes
/// ```
/// use tycho::{Element, Value, decode, to_bytes};
/// use serde::Serialize;
///
/// #[derive(Serialize, Debug)]
/// pub struct Example {
///     foo: String,
///     bar: u8,
///     baz: bool
/// }
///
/// let element = Example {
///     foo: "Hello World".to_string(),
///     bar: 42,
///     baz: true
/// };
///
/// let bytes = to_bytes(&element).unwrap();
/// // assert_eq!(bytes, vec![64, 3, 3, 102, 111, 111, 29, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 3, 98, 97, 114, 17, 42, 3, 98, 97, 122, 16, 1])
/// ```
pub fn to_bytes<T: Serialize>(o: &T) -> Result<Vec<u8>, SerializeError> {
    Ok(encode(o.serialize(TychoSerializer)?))
}

/// Deserialize from an element
pub fn from_element<T: DeserializeOwned>(data: Element) -> Result<T, DeserializeError> {
    T::deserialize(de::de::TychoDeserializer::new(data))
}
/// Serialize from into an element
pub fn to_element<T: Serialize>(o: &T) -> Result<Element, SerializeError> {
    o.serialize(TychoSerializer)
}

#[cfg(test)]
mod tests {
    use crate::{to_bytes, from_bytes};
    use serde::{Serialize, Deserialize};
    use crate::decode::decode;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct Example {
        foo: String,
        bar: bool,
        baz: Option<u16>,
        cum: Vec<bool>
    }

    #[test]
    fn serde_example() {
        let example = Example {
            foo: "Hello World!".to_string(),
            bar: true,
            baz: Some(10),
            cum: vec![true, false, true, false, false, true]
        };
        let data = to_bytes(&example).unwrap();
        let result: Example = from_bytes(data).unwrap();
        assert_eq!(example, result)
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum ExampleEnum {
        A,
        B (u8),
        C (u8, u16, u32),
        D { one: bool, two: String }
    }

    fn test_enum(input: ExampleEnum) {
        let data = to_bytes(&input).unwrap();
        let result: ExampleEnum = from_bytes(data).unwrap();
        assert_eq!(input, result)
    }

    #[test]
    fn serde_enum_example() {
        test_enum(ExampleEnum::A);
        test_enum(ExampleEnum::B(16));
        test_enum(ExampleEnum::C(1, 2, 3));
        test_enum(ExampleEnum::D { one: false, two: "Hi".to_string() });
    }

    #[test]
    fn error_fixing() {
        let data = std::fs::read("com.key").unwrap();
        println!("{:?}", decode(data))
    }
}