use std::io::{Read, Cursor, Write, Seek, Error as IoError};
use crate::collections::Struct;
use crate::{marshall_vec, Value};
use byteorder::ReadBytesExt;
use crate::read::element::read_element_ident;
use crate::error::{TychoError, TychoResult};
use crate::types::ident::ElementIdent;
use crate::read::value::{read_value_ident, read_value};
use crate::read::length::read_length;


pub struct PartialReader<R: Read + Write> {
    reader: R,
    pointer: u64
}

impl<R: Read + Write> PartialReader<R> {

}

#[derive(Debug, Clone)]
pub struct PartialPointer {
    pub head: usize,
    pub size: usize
}

#[derive(Debug, Clone)]
pub enum PartialElement {
    Unit,
    Value(Value),
    Struct(PartialStruct)
}

impl PartialElement {
    pub fn read<R: Read + Seek>(reader: &mut R) -> TychoResult<Self> {
        let ident = read_element_ident(reader)?;

        match ident {
            ElementIdent::Unit => Ok(PartialElement::Unit),
            ElementIdent::Value => {
                let prefix = read_value_ident(reader)?;
                let value = read_value(reader, &prefix)?;
                Ok(PartialElement::Value(value))
            }
            ElementIdent::Struct => {
                let size = read_length(reader)?;
                reader.
            }

            _ => { panic!("{:?}", ident)}
        }
    }
}

#[derive(Debug, Clone)]
pub struct PartialStruct {
    pub pointer: PartialPointer
}

impl PartialStruct {
    pub fn next<R: Read + Seek>(&self, reader: &mut PartialReader<R>) -> IoResult<Option<(String, PartialElement)>> {
        reader.reader.read_u8()
    }
}


#[test]
fn test() {
    let mut data = Struct::new();

    data.insert("foo", 10u8);
    data.insert("bar", 20u16);
    data.insert("baz", "Hello World");

    println!("{:?}", data);

    let bytes = marshall_vec(data).unwrap();

    println!("{:?}", bytes);

    let mut reader = Cursor::new(bytes);


}
