use std::io::{Seek, Read};
use crate::partial::types::PartialStruct;
use crate::Value;
use crate::partial::reader::PartialReader;
use crate::error::TychoResult;
use crate::types::ident::{ElementIdent, ValueIdent};
use crate::read::value::{read_value_ident, read_value};
use crate::read::element::read_element_ident;
use crate::read::length::read_length;
use crate::read::string::read_tstring;
use crate::partial::{PartialArray, PartialList, PartialMap};

#[derive(Debug, Clone)]
pub enum PartialElement {
    Unit,
    Value(Value),
    Option(Option<Box<PartialElement>>),
    Variant(String, Box<PartialElement>),
    Struct(PartialStruct),
    List(PartialList),
    Map(PartialMap),
    Array(PartialArray),

}

impl PartialElement {
    pub fn read<R: Read + Seek>(reader: &mut PartialReader<R>) -> TychoResult<Self> {
        let ident = read_element_ident(reader)?;

        match ident {
            ElementIdent::Unit => Ok(PartialElement::Unit),
            ElementIdent::Value => {
                let prefix = read_value_ident(reader)?;
                let value = read_value(reader, &prefix)?;
                Ok(PartialElement::Value(value))
            },

            ElementIdent::None => Ok(PartialElement::Option(None)),
            ElementIdent::Some => PartialElement::read(reader),

            ElementIdent::Variant => {
                let name = read_tstring(reader)?;
                let value = PartialElement::read(reader)?;

                Ok(PartialElement::Variant(name, Box::new(value)))
            }

            ElementIdent::Struct => {
                let size = read_length(reader)? as u64;
                let pos = reader.pointer.clone();
                reader.jump(&size)?;
                Ok(PartialElement::Struct(PartialStruct::new(reader.pointer(pos, size), 0, ())))
            },

            ElementIdent::List => {
                let size = read_length(reader)? as u64;
                let pos = reader.pointer.clone();
                reader.jump(&size)?;
                Ok(PartialElement::List(PartialList::new(reader.pointer(pos, size), 0, ())))
            },

            ElementIdent::Array => {
                let array_type = read_value_ident(reader)?;

                if array_type == ValueIdent::Null {
                    return Ok(PartialElement::Array(PartialArray::empty(reader.empty_pointer(),array_type)))
                }

                let size = read_length(reader)? as u64;
                let pos = reader.pointer.clone();
                reader.jump(&size)?;
                Ok(PartialElement::Array(PartialArray::new(reader.pointer(pos, size), 0, array_type)))
            },

            ElementIdent::Map => {
                let key_type = read_value_ident(reader)?;

                if key_type == ValueIdent::Null {
                    return Ok(PartialElement::Map(PartialMap::empty(reader.empty_pointer(), key_type)))
                }

                let size = read_length(reader)? as u64;
                let pos = reader.pointer.clone();
                reader.jump(&size)?;
                Ok(PartialElement::Map(PartialMap::new(reader.pointer(pos, size), 0, key_type)))
            },

            _ => { panic!("{:?}", ident)}
        }
    }
}
