use std::io::{BufWriter, Write};

use byteorder::WriteBytesExt;

use crate::error::{TychoError, TychoStatus};
use crate::write::length::write_length;

pub(crate) fn write_byte<W: Write>(writer: &mut W, byte: &u8) -> TychoStatus {
    match writer.write_u8(*byte) {
        Ok(_) => Ok(()),
        Err(e) => Err(TychoError::from(e))
    }
}

pub(crate) fn write_bytes<W: Write>(writer: &mut W, bytes: &[u8]) -> TychoStatus {
    match writer.write_all(bytes) {
        Ok(_) => Ok(()),
        Err(e) => Err(TychoError::from(e))
    }
}

pub(crate) fn write_buffer<W: Write>(writer: &mut W, buffer: BufWriter<Vec<u8>>) -> TychoStatus {
    let bytes = buffer.into_inner().unwrap(); // issue here?
    write_length(writer, bytes.len())?;
    write_bytes(writer, &bytes)
}