use std::io::Write;
use crate::{Value};
use crate::write::func::{write_byte, write_bytes};
use crate::write::number::{write_number_ident, write_number};
use crate::write::string::{write_string, write_char};
use crate::write::length::write_length;
use crate::types::ident::ValueIdent;
use crate::error::TychoStatus;

pub(crate) fn write_value_ident<W: Write>(writer: &mut W, ident: &ValueIdent) -> TychoStatus {
    match ident {
        ValueIdent::Null => write_byte(writer, &0x00),
        ValueIdent::Boolean => write_byte(writer, &0x01),
        ValueIdent::String => write_byte(writer, &0x02),
        ValueIdent::Char => write_byte(writer, &0x03),
        ValueIdent::Number(num) => {
            write_byte(writer, &0x04)?;
            write_number_ident(writer, num)
        },
        ValueIdent::Bytes => write_byte(writer, &0x05),
        ValueIdent::UUID => write_byte(writer, &0x06),
    }
}

pub(crate) fn write_value<W: Write>(writer: &mut W, value: &Value) -> TychoStatus {
    match value {
        Value::Null => Ok(()),
        Value::Boolean(v) =>
            if v == &true { write_byte(writer, &0x01) }
            else { write_byte(writer, &0x00) }
        Value::String(v) => write_string(writer, v),
        Value::Char(v) => write_char(writer, v),
        Value::Number(v) => write_number(writer, v),
        Value::Bytes(v) => {
            write_length(writer, v.len())?;
            write_bytes(writer, v)
        }
        Value::UUID(uuid) => write_bytes(writer, &uuid.as_bytes().as_ref())
    }
}