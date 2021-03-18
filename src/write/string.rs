use std::io::Write;

use crate::error::TychoStatus;
use crate::write::func::{write_byte, write_bytes};
use crate::write::length::write_length;

pub(crate) fn write_string<W: Write>(writer: &mut W, s: &str) -> TychoStatus {
    let bytes = s.as_bytes();

    write_length(writer, bytes.len())?;
    write_bytes(writer, bytes)
}

pub(crate) fn write_tstring<W: Write>(writer: &mut W, s: &str) -> TychoStatus {
    write_bytes(writer, &s.as_bytes())?;
    write_byte(writer, &0x00)
}

pub(crate) fn write_char<W: Write>(writer: &mut W, c: &char) -> TychoStatus {
    write_bytes(writer, &c.to_string().as_bytes())
}