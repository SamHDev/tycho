use std::io::{Read, Cursor};
use crate::{TychoResult, ValueIdent, Element, ElementIdent, TychoError};
use crate::read::func::{read_byte, read_bytes};
use crate::read::value::{read_value, read_value_ident};
use crate::read::string::read_tstring;
use crate::read::length::read_length;
use std::collections::HashMap;

pub(crate) fn read_element_ident<R: Read>(reader: &mut R) -> TychoResult<ElementIdent> {
    let byte = read_byte(reader)?;

    match byte {
        0x00 => Ok(ElementIdent::Unit),
        0x01 => Ok(ElementIdent::Value),
        0x02 => Ok(ElementIdent::None),
        0x03 => Ok(ElementIdent::Some),
        0x04 => Ok(ElementIdent::Variant),
        0x05 => Ok(ElementIdent::Struct),
        0x06 => Ok(ElementIdent::List),
        0x07 => Ok(ElementIdent::Array),
        0x08 => Ok(ElementIdent::Map),

        0xF0 => Ok(ElementIdent::Compression),

        _ => Err(TychoError::InvalidIdent { found: byte, expecting: "element ident".to_string() })
    }
}

pub(crate) fn read_element<R: Read>(reader: &mut R) -> TychoResult<Element> {
    let ident = read_element_ident(reader)?;

    match ident {
        ElementIdent::Unit => Ok(Element::Unit),
        ElementIdent::Value => {
            let ident = read_value_ident(reader)?;
            Ok(Element::Value(read_value(reader, &ident)?))
        },
        ElementIdent::Some => Ok(Element::Option(Some(Box::new(read_element(reader)?)))),
        ElementIdent::None => Ok(Element::Option(None)),
        ElementIdent::Variant => Ok(Element::Variant(
            read_tstring(reader)?,
            Box::new(read_element(reader)?)
        )),
        ElementIdent::Struct => {
            let size = read_length(reader)?;
            let mut items = HashMap::new();
            let mut buffer = Cursor::new(read_bytes(reader, size)?);

            loop {
                if buffer.position() == size as u64 { break; }

                let key = read_tstring(&mut buffer)?;
                let value = read_element(&mut buffer)?;

                items.insert(key, value);
            }

            Ok(Element::Struct(items))
        }
        ElementIdent::List => {
            let size = read_length(reader)?;
            let mut items = Vec::new();
            let mut buffer = Cursor::new(read_bytes(reader, size)?);

            loop {
                if buffer.position() == size as u64 { break; }
                items.push(read_element(&mut buffer)?);
            }

            Ok(Element::List(items))
        },
        ElementIdent::Array => {
            let array_type = read_value_ident(reader)?;

            if let ValueIdent::Null = &array_type {
                Ok(Element::Array(ValueIdent::Null, Vec::new()))
            } else {
                let size = read_length(reader)?;
                let mut items = Vec::new();
                let mut buffer = Cursor::new(read_bytes(reader, size)?);

                loop {
                    if buffer.position() == size as u64 { break; }
                    items.push(read_value(&mut buffer, &array_type)?);
                }

                Ok(Element::Array(array_type, items))
            }
        },
        ElementIdent::Map => {
            let key_type = read_value_ident(reader)?;

            if let ValueIdent::Null = &key_type {
                Ok(Element::Map(ValueIdent::Null, HashMap::new()))
            } else {
                let size = read_length(reader)?;
                let mut items = HashMap::new();
                let mut buffer = Cursor::new(read_bytes(reader, size)?);

                loop {
                    if buffer.position() == size as u64 { break; }

                    let key = read_value(&mut buffer, &key_type)?;
                    let value = read_element(&mut buffer)?;

                    items.insert(key, value);
                }

                Ok(Element::Map(key_type, items))
            }
        },
        ElementIdent::Compression => {
            let size = read_length(reader)?;
            let mut buffer = Cursor::new(read_bytes(reader, size)?);
            Ok(Element::Compression(Box::new(read_element(&mut buffer)?)))
        }
    }
}

