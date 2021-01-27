pub mod encode;
pub mod util;
mod values;
mod elements;
mod ident;
pub mod decode;
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

pub fn encode(element: Element) -> Vec<u8> {
    element.encode()
}

pub fn decode(data: Vec<u8>) -> Result<Element, DecodeError> {
    decode::decode(data)
}

pub fn from_bytes<T: DeserializeOwned>(data: Vec<u8>) -> Result<T, DeserializeError> {
    match decode(data) {
        Ok(element) =>  T::deserialize(de::de::TychoDeserializer::new(element)),
        Err(e) => Err(DeserializeError::DecodeError(e))
    }
}

pub fn to_bytes<T: Serialize>(o: &T) -> Result<Vec<u8>, SerializeError> {
    Ok(encode(o.serialize(TychoSerializer)?))
}

pub fn from_element<T: DeserializeOwned>(data: Element) -> Result<T, DeserializeError> {
    T::deserialize(de::de::TychoDeserializer::new(data))
}

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