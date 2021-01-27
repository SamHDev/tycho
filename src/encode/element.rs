use crate::{Element, Value};
use crate::ident::{ElementIdent, ValueIdent};
use crate::encode::util::{prefix_bytes, join_idents, join_nibs, one_ident, join_bytes};
use crate::encode::value::ValueEncoder;
use crate::encode::length::encode_variable_length;
use crate::encode::string::{encode_string};
use std::collections::HashMap;

pub trait ElementEncoder {
    fn ident(&self) -> ElementIdent;
    fn prefix(&self) -> u8;
    fn body(&self) -> Vec<u8>;
    fn encode(&self) -> Vec<u8> {
        prefix_bytes(self.prefix(), self.body())
    }
}

impl ElementEncoder for Element {
    fn ident(&self) -> ElementIdent {
        match &self {
            Element::Unit => ElementIdent::Unit,
            Element::Value(_) => ElementIdent::Value,
            Element::Option(_) => ElementIdent::Option,
            Element::Array(_) => ElementIdent::Array,
            Element::Struct(_) => ElementIdent::Struct,
            Element::Variant(_, _) => ElementIdent::Variant,
            Element::Map(_) => ElementIdent::Map,
            Element::List(_) => ElementIdent::List
        }
    }

    fn prefix(&self) -> u8 {
        match &self {
            Element::Unit => one_ident(self.ident()),
            Element::Value(value) => join_idents(self.ident(), value.ident()),
            Element::Option(opt) =>
                join_nibs(self.ident() as u8, opt.is_some() as u8),

            Element::Array(_) => one_ident(self.ident()),
            Element::Struct(_) => one_ident(self.ident()),
            Element::Variant(_, _) => one_ident(self.ident()),

            Element::Map(map) =>
                join_idents(
                    self.ident(),
                    derive_map_key_type(&map).unwrap_or(ValueIdent::NIL)
                ),
            Element::List(list) =>
                join_idents(
                    self.ident(),
                    derive_list_type(&list).unwrap_or(ValueIdent::NIL)
                )
        }
    }

    fn body(&self) -> Vec<u8> {
        match &self {
            Element::Unit => Vec::new(),
            Element::Value(value) => value.body(),

            Element::Option(opt) =>  match opt {
                Some(value) => value.encode(),
                None => Vec::new()
            }

            Element::Array(array) => {
                let mut build = encode_variable_length(array.len() as u32);
                for item in array {
                    build.extend_from_slice(&item.encode())
                }
                build
            }
            Element::Struct(structure) => {
                let mut build = encode_variable_length(structure.len() as u32);
                for (key, value) in structure {
                    build.extend_from_slice(&encode_string(key));
                    build.extend_from_slice(&value.encode());
                }
                build
            }
            Element::Variant(name, inner) => {
                join_bytes(encode_string(&name), &inner.encode())
            }
            Element::Map(map) => {
                let mut build = encode_variable_length(map.len() as u32);
                for (key, value) in map {
                    build.extend_from_slice(&key.body());
                    build.extend_from_slice(&value.encode());
                }
                build
            }
            Element::List(list) => {
                let mut build = encode_variable_length(list.len() as u32);
                for element in list {
                    build.extend_from_slice(&element.body())
                }
                build
            }
        }
    }

}

fn derive_map_key_type(map: &HashMap<Value, Element>) -> Option<ValueIdent> {
    match map.iter().nth(0) {
        Some((key, _)) => {
            Some(key.ident())
        },
        None => {
            None
        }
    }
}

fn derive_list_type(list: &Vec<Value>) -> Option<ValueIdent> {
    match list.iter().nth(0) {
        Some(value) => {
            Some(value.ident())
        },
        None => {
            None
        }
    }
}


#[cfg(test)]
mod encode_tests {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use crate::Element;
    use crate::util::{ElementUtil, ValueUtil};
    use crate::encode::element::ElementEncoder;

    #[test]
    fn encode_unit() {
        assert_eq!(Element::Unit.encode(), vec![0])
    }

    #[test]
    fn encode_value_bool() {
        assert_eq!(Element::bool(true).encode(), vec![16, 1]);
        assert_eq!(Element::bool(false).encode(), vec![16, 0]);
    }

    #[test]
    fn encode_value_string() {
        assert_eq!(Element::string("Hello World").encode(), vec![29, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100])
    }

    #[test]
    fn encode_value_char() {
        assert_eq!(Element::char('H').encode(), vec![30, 72, 0]);
        assert_eq!(Element::char('🍆').encode(), vec![30, 240, 159, 141, 134, 0]);
    }

    #[test]
    fn encode_value_bytes() {
        assert_eq!(Element::bytes(&vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]).encode(), vec![31, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    }

    #[test]
    fn encode_value_unsigned() {
        assert_eq!(Element::u8(42).encode(), vec![17, 42]);
        assert_eq!(Element::u16(25656).encode(), vec![18, 100, 56]);
        assert_eq!(Element::u32(263871623).encode(), vec![19, 15, 186, 92, 135]);
        assert_eq!(Element::u64(278982917381212312).encode(), vec![20, 3, 223, 37, 124, 56, 232, 200, 152]);
        assert_eq!(Element::u128(278982917323821387128937123181212312).encode(), vec![21, 0, 53, 186, 235, 119, 155, 191, 206, 238, 145, 248, 53, 15, 223, 178, 152]);
    }

    #[test]
    fn encode_value_signed() {
        assert_eq!(Element::i8(-42).encode(), vec![22, 214]);
        assert_eq!(Element::i16(-25656).encode(), vec![23, 155, 200]);
        assert_eq!(Element::i32(-263871623).encode(), vec![24, 240, 69, 163, 121]);
        assert_eq!(Element::i64(-278982917381212312).encode(), vec![25, 252, 32, 218, 131, 199, 23, 55, 104]);
        assert_eq!(Element::i128(-278982917323821387128937123181212312).encode(), vec![26, 255, 202, 69, 20, 136, 100, 64, 49, 17, 110, 7, 202, 240, 32, 77, 104]);
    }

    #[test]
    fn encode_value_floating() {
        assert_eq!(Element::f32(420.0).encode(), vec![27, 67, 210, 0, 0]);
        assert_eq!(Element::f32(420.69).encode(), vec![27, 67, 210, 88, 82]);
        assert_eq!(Element::f32(-420.69).encode(), vec![27, 195, 210, 88, 82]);
        assert_eq!(Element::f64(420000000000000.0).encode(), vec![28, 66, 247, 223, 205, 236, 228, 0, 0]);
        assert_eq!(Element::f64(420.69696969696969696969696969696969).encode(), vec![28, 64, 122, 75, 38, 201, 178, 108, 155]);
        assert_eq!(Element::f64(-420.69696969696969696996969969).encode(), vec![28, 192, 122, 75, 38, 201, 178, 108, 155]);
    }

    #[test]
    fn encode_option() {
        assert_eq!(Element::some(10_u8).encode(), vec![33, 17, 10]);
        assert_eq!(Element::none().encode(), vec![32]);
    }

    #[test]
    fn encode_array() {
        assert_eq!(Element::array(vec![10_u8, 20, 30]).encode(), vec![48, 3, 17, 10, 17, 20, 17, 30]);
        assert_eq!(Element::array(vec![
            Element::u8(10),
            Element::string("gamer_time"),
            Element::some(false)]).encode(), vec![48, 3, 17, 10, 29, 10, 103, 97, 109, 101, 114, 95, 116, 105, 109, 101, 33, 16, 0]);
        assert_eq!(Element::array(vec![
            Element::array(vec![ Element::u8(10),
                                 Element::string("gamer_time"),
                                 Element::some(false)]),
            Element::string("gamer_time_2")
        ]).encode(), vec![48, 2, 48, 3, 17, 10, 29, 10, 103, 97, 109, 101, 114, 95, 116, 105, 109, 101, 33, 16, 0, 29, 12, 103, 97, 109, 101, 114, 95, 116, 105, 109, 101, 95, 50])
    }

    #[test]
    fn encode_list() {
        assert_eq!(Element::list(vec![10_u8, 20, 30]).encode(), vec![113, 3, 10, 20, 30]);
    }

    #[test]
    fn encode_variant() {
        // TODO: ACTUALLY TEST MORE
        assert_eq!(Element::list(vec![10_u8, 20, 30]).encode(), vec![113, 3, 10, 20, 30]);
    }
}