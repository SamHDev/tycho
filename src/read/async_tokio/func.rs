use std::io::Read;
use byteorder::ReadBytesExt;
use crate::error::{TychoResult, parse_io};

pub(crate) fn read_byte<R: Read>(reader: &mut R) -> TychoResult<u8> {
    parse_io(reader.read_u8())
}

pub(crate) fn read_bytes<R: Read>(reader: &mut R, size: usize) -> TychoResult<Vec<u8>> {
    let mut buffer = Vec::with_capacity(size);
    for _ in 0..size {
        buffer.push(read_byte(reader)?);
    }
    Ok(buffer)
}