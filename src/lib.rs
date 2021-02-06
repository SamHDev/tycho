mod encode;
mod util;
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
}