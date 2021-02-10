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
    fn debug_error() {
        let data = vec![80, 11, 105, 110, 105, 116, 46, 115, 101, 114, 118, 101, 114, 64, 1, 3, 107, 101, 121, 64, 2, 1, 101, 113, 3, 1, 0, 1, 1, 110, 113, 64, 128, 198, 130, 157, 122, 135, 212, 252, 72, 207, 117, 60, 214, 157, 234, 185, 13, 99, 20, 155, 56, 1, 36, 153, 4, 59, 225, 82, 226, 148, 12, 73, 89, 121, 125, 175, 42, 209, 162, 48, 228, 255, 184, 198, 239, 43, 69, 113, 255, 40, 122, 109, 132, 137, 61, 247, 54, 84, 132, 247, 81, 110, 66, 109, 164, 98, 73, 202, 32, 117, 143, 43, 71, 131, 199, 171, 138, 53, 87, 157, 195, 180, 209, 59, 189, 254, 231, 245, 144, 189, 53, 162, 59, 39, 17, 153, 115, 69, 50, 86, 5, 222, 100, 43, 227, 111, 151, 87, 139, 142, 117, 135, 246, 121, 130, 34, 104, 17, 29, 185, 143, 87, 220, 168, 18, 221, 152, 164, 1];

        println!("{:?}", decode(data));
    }
}