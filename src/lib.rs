pub(crate) mod types;
pub(crate) mod collections;

pub(crate) mod write;
pub(crate) mod read;
pub(crate) mod error;
pub(crate) mod into;
pub(crate) mod public;


pub use types::ident::*;
pub use types::types::*;
pub use error::*;
pub use public::*;


#[cfg(test)]
mod tests {
    use crate::{Element, Value, to_bytes, from_bytes, Number};
    use std::collections::HashMap;

    #[test]
    fn encode_0() {
        let element = Element::Value(Value::String("Hello World".to_string()));
        println!("{:?}", element);
        println!("{:?}", to_bytes(&element))
    }

    #[test]
    fn decode_0() {
        let data = vec![1, 2, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100];

        println!("{:?}", from_bytes(data));
    }

    #[test]
    fn encode_1() {
        let mut map = HashMap::new();
        map.insert("foo".to_string(), Element::Value(Value::Boolean(true)));
        map.insert("bar".to_string(), Element::Value(Value::Number(Number::Unsigned8(42))));
        map.insert("baz".to_string(), Element::Value(Value::String("Hello".to_string())));

        let element = Element::Struct(map);
        println!("{:?}", element);
        println!("{:?}", to_bytes(&element))
    }

    #[test]
    fn decode_1() {
        let data = vec![5, 27, 102, 111, 111, 0, 1, 1, 1, 98, 97, 114, 0, 1, 4, 1, 42, 98, 97, 122, 0, 1, 2, 5, 72, 101, 108, 108, 111];

        println!("{:?}", from_bytes(data));
    }
}