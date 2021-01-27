use crate::decode::reader::Reader;
use crate::decode::error::DecodeError;
use crate::decode::length::read_var_length;

pub(crate) fn read_string(reader: &mut Reader) -> Result<String, DecodeError> {
    let length = read_var_length(reader)?;
    let bytes = reader.read_many(length as usize)?;
    match String::from_utf8(bytes.clone()) {
        Ok(s) => Ok(s),
        Err(e) => Err(DecodeError::StringDecodeError { pos: reader.pos() - bytes.len(), error: e})
    }
}

pub(crate) fn read_term_string(reader: &mut Reader) -> Result<String, DecodeError> {
    let mut bytes = Vec::new();
    loop {
        let byte = reader.read_one()?;
        if byte == 0x00 {
            break;
        } else {
            bytes.push(byte);
        }
    }
    match String::from_utf8(bytes.clone()) {
        Ok(s) => Ok(s),
        Err(e) => Err(DecodeError::StringDecodeError { pos: reader.pos() - bytes.len(), error: e})
    }
}