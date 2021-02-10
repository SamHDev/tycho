use crate::decode::reader::Reader;
use crate::ident::{ElementIdent, Ident, ValueIdent};
use crate::decode::error::DecodeError;
use crate::Element;
use crate::decode::value::decode_value;
use crate::decode::length::read_var_length;
use std::collections::HashMap;
use crate::decode::string::read_string;
use std::process::id;

pub(crate) fn decode_joint_prefix(reader: &mut Reader) -> Result<(ElementIdent, u8), DecodeError> {
    let byte = reader.read_one()?;
    let (a, b) = (byte >> 4, byte & 0xF);

    match ElementIdent::parse(&a) {
        None => return Err(DecodeError::InvalidElementIdent { pos: reader.pos() - 1, value: a}),
        Some(ident) => Ok((ident, b))
    }
}

pub(crate) fn decode_prefixed_element(reader: &mut Reader)  -> Result<Element, DecodeError> {
    let (a, b) = decode_joint_prefix(reader)?;
    decode_element(a, b, reader)
}

pub(crate) fn decode_element(ident: ElementIdent, prefix_data: u8, reader: &mut Reader) -> Result<Element, DecodeError> {
    println!("{:?}", &ident);
    match ident {
        ElementIdent::Unit => Ok(Element::Unit),
        ElementIdent::Value => match ValueIdent::parse(&prefix_data) {
            None => Err(DecodeError::InvalidValueIdent { pos: reader.pos() - 1, value: prefix_data }),
            Some(value_ident) => Ok(Element::Value(decode_value(value_ident, reader)?))
        }
        ElementIdent::Option => match &prefix_data {
            0x00 => Ok(Element::Option(None)),
            0x01 => Ok(Element::Option(Some(Box::new(decode_prefixed_element(reader)?)))),
            _ => Err(DecodeError::InvalidElementOperand { pos: reader.pos() - 1, value: prefix_data })
        }
        ElementIdent::Array => {
            let length = read_var_length(reader)?;
            let mut array = Vec::new();
            for _ in 0..length {
                array.push(decode_prefixed_element(reader)?)
            }
            Ok(Element::Array(array))
        }
        ElementIdent::Struct => {
            let length = read_var_length(reader)?;
            let mut structure = HashMap::new();
            for _ in 0..length {
                let key = read_string(reader)?;
                let value = decode_prefixed_element(reader)?;
                structure.insert(key, value);
            }
            Ok(Element::Struct(structure))
        }
        ElementIdent::Variant => {
            let name = read_string(reader)?;
            let inner = decode_prefixed_element(reader)?;
            Ok(Element::Variant(name, Box::new(inner)))
        }
        ElementIdent::Map => {
            if prefix_data == ValueIdent::NIL.ident() {
                return Ok(Element::Map(HashMap::new()));
            }
            let key_type = match ValueIdent::parse(&prefix_data) {
                None => return Err(DecodeError::InvalidValueIdent { pos: reader.pos() - 1, value: prefix_data }),
                Some(value_ident) => value_ident
            };

            let length = read_var_length(reader)?;
            let mut map = HashMap::new();
            for _ in 0..length {
                let key = decode_value(key_type.clone(), reader)?;
                let value = decode_prefixed_element(reader)?;
                map.insert(key, value);
            }
            Ok(Element::Map(map))
        }
        ElementIdent::List => {
            println!("LIST?");
            if prefix_data == ValueIdent::NIL.ident() {
                return Ok(Element::List(Vec::new()));
            }
            let key_type = match ValueIdent::parse(&prefix_data) {
                None => return Err(DecodeError::InvalidValueIdent { pos: reader.pos() - 1, value: prefix_data }),
                Some(value_ident) => value_ident
            };
            let length = read_var_length(reader)?;
            println!("{:?} {:?}", &key_type, length);
            let mut list = Vec::new();
            for _ in 0..length {
                let value = decode_value(key_type.clone(), reader)?;
                list.push(value);
            }
            Ok(Element::List(list))
        }
    }
}

