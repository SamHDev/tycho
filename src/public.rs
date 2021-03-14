use std::io::{Write, BufWriter, Read, Cursor};
use crate::{Element};
use crate::write::element::write_element;
use crate::read::element::read_element;
use crate::error::{TychoStatus, TychoResult};

pub fn marshall<W: Write, E: Into<Element>>(writer: &mut W, element: E) -> TychoStatus {
    write_element(writer, &element.into())
}

pub fn marshall_vec<E: Into<Element>>(element: E) -> TychoResult<Vec<u8>> {
    let mut buffer = BufWriter::new(Vec::new());
    marshall(&mut buffer, element)?;
    Ok(buffer.into_inner().unwrap()) // todo: issue may occur here not sure. will catch later.
}

pub fn unmarshall<R: Read>(reader: &mut R) -> TychoResult<Element> {
    read_element(reader)
}

pub fn unmarshall_vec(data: Vec<u8>) -> TychoResult<Element>  {
    let mut buffer = Cursor::new(data);
    unmarshall(&mut buffer)
}

