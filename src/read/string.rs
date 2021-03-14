use std::io::Read;
use crate::read::length::read_length;
use crate::read::func::{read_bytes, read_byte};
use crate::error::{TychoResult, TychoError};

pub(crate) fn read_string<R: Read>(reader: &mut R) -> TychoResult<String> {
    let length = read_length(reader)?;
    match String::from_utf8(read_bytes(reader, length)?) {
        Ok(s) => Ok(s),
        Err(e) => Err(TychoError::StringError(e))
    }
}

pub(crate) fn read_tstring<R: Read>(reader: &mut R) -> TychoResult<String> {
    let mut buffer = Vec::new();
    loop {
        let byte = read_byte(reader)?;
        if byte == 0x00 {
            break;
        }
        buffer.push(byte);
    }
    match String::from_utf8(buffer) {
        Ok(s) => Ok(s),
        Err(e) => Err(TychoError::StringError(e))
    }
}

pub(crate) fn read_char<R: Read>(reader: &mut R) -> TychoResult<char> {
    let mut buffer = Vec::new();

    let byte = read_byte(reader)?;
    if byte >> 7 == 0 {
        buffer.push(byte);
    } else {
        let count = if byte & 0b01000000 == 0x00 { 1 }
        else if byte & 0b00100000 == 0x00 { 2 }
        else if byte & 0b00010000 == 0x00 { 3 }
        else if byte & 0b00001000 == 0x00 { 4 }
        else if byte & 0b00000100 == 0x00 { 5 }
        else if byte & 0b00000010 == 0x00 { 6 }
        else { 0 };

        buffer.extend_from_slice(&read_bytes(reader, count)?);
    }


    match String::from_utf8(buffer) {
        Ok(s) => Ok(s.chars().nth(0).unwrap().clone()), //todo: unwrap :(
        Err(e) => Err(TychoError::StringError(e))
    }
}