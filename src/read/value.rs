use crate::{ValueIdent, TychoResult, TychoError, Value};
use std::io::Read;
use crate::read::func::{read_byte, read_bytes};
use crate::read::number::{read_number_ident, read_number};
use crate::read::string::{read_string, read_char};
use crate::read::length::read_length;
use uuid::Uuid;

pub(crate) fn read_value_ident<R: Read>(reader: &mut R) -> TychoResult<ValueIdent> {
    let byte = read_byte(reader)?;

    match byte {
        0x00 => Ok(ValueIdent::Null),
        0x01 => Ok(ValueIdent::Boolean),
        0x02 => Ok(ValueIdent::String),
        0x03 => Ok(ValueIdent::Char),
        0x04 => Ok(ValueIdent::Number(read_number_ident(reader)?)),
        0x05 => Ok(ValueIdent::Bytes),
        0x06 => Ok(ValueIdent::UUID),

        _ => Err(TychoError::InvalidIdent { found: byte, expecting: "value ident".to_string() })
    }
}

pub(crate) fn read_value<R: Read>(reader: &mut R, ident: &ValueIdent) -> TychoResult<Value> {
    match ident {
        ValueIdent::Null => Ok(Value::Null),
        ValueIdent::Boolean => Ok(Value::Boolean(read_byte(reader)? == 0x01)),
        ValueIdent::String => Ok(Value::String(read_string(reader)?)),
        ValueIdent::Char => Ok(Value::Char(read_char(reader)?)),
        ValueIdent::Number(n) => Ok(Value::Number(read_number(reader, n)?)),
        ValueIdent::Bytes => {
            let length = read_length(reader)?;
            Ok(Value::Bytes(read_bytes(reader, length)?))
        }
        ValueIdent::UUID => {
            let bytes = [
                read_byte(reader)?, read_byte(reader)?, read_byte(reader)?, read_byte(reader)?,
                read_byte(reader)?, read_byte(reader)?, read_byte(reader)?, read_byte(reader)?,
                read_byte(reader)?, read_byte(reader)?, read_byte(reader)?, read_byte(reader)?,
                read_byte(reader)?, read_byte(reader)?, read_byte(reader)?, read_byte(reader)?,
            ];
            Ok(Value::UUID(Uuid::from_bytes(bytes)))
        }
    }
}