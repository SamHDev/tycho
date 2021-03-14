use std::io::{Write, BufWriter, Read, Cursor};
use crate::{Element, TychoResult, TychoStatus};
use crate::write::element::write_element;
use crate::read::element::read_element;

pub fn write<W: Write>(writer: &mut W, element: &Element) -> TychoStatus {
    write_element(writer, &element)
}

pub fn to_bytes(element: &Element) -> TychoResult<Vec<u8>> {
    let mut buffer = BufWriter::new(Vec::new());
    write(&mut buffer, &element)?;
    Ok(buffer.into_inner().unwrap()) // todo: issue may occur here not sure. will catch later.
}

pub fn read<R: Read>(reader: &mut R) -> TychoResult<Element> {
    read_element(reader)
}

pub fn from_bytes(data: Vec<u8>) -> TychoResult<Element>  {
    let mut buffer = Cursor::new(data);
    read(&mut buffer)
}