use std::io::{Seek, Read};
use crate::partial::types::PartialStruct;
use crate::Value;
use crate::partial::reader::PartialReader;
use crate::error::TychoResult;
use crate::types::ident::ElementIdent;
use crate::read::value::{read_value_ident, read_value};
use crate::read::element::read_element_ident;
use crate::read::length::read_length;

#[derive(Debug, Clone)]
pub enum PartialElement {
    Unit,
    Value(Value),
    Struct(PartialStruct)
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
            }
            ElementIdent::Struct => {
                let size = read_length(reader)? as u64;
                let pos = reader.pointer.clone();
                reader.jump(&size)?;
                Ok(PartialElement::Struct(PartialStruct::new(pos, size, 0)))
            }

            _ => { panic!("{:?}", ident)}
        }
    }
}
