use crate::{Value};
use std::io::Read;
use uuid::Uuid;
use crate::ident::ValueIdent;
use crate::error::{TychoResult, TychoError};
use crate::read::async_tokio::func::{read_byte_async, read_bytes_async};
use tokio::io::AsyncRead;
use crate::read::async_tokio::number::{read_number_ident_async, read_number_async};
use crate::read::async_tokio::string::{read_string_async, read_char_async};
use crate::read::length::read_length;
use crate::read::async_tokio::length::read_length_async;

pub(crate) fn read_value_ident_async<R: AsyncRead>(reader: &mut R) -> TychoResult<ValueIdent> {
    let byte = read_byte_async(reader).await?;

    match byte {
        0x00 => Ok(ValueIdent::Null),
        0x01 => Ok(ValueIdent::Boolean),
        0x02 => Ok(ValueIdent::String),
        0x03 => Ok(ValueIdent::Char),
        0x04 => Ok(ValueIdent::Number(read_number_ident_async(reader).await?)),
        0x05 => Ok(ValueIdent::Bytes),
        0x06 => Ok(ValueIdent::UUID),

        _ => Err(TychoError::InvalidIdent { found: byte, expecting: "value ident".to_string() })
    }
}

pub(crate) fn read_value_async<R: Read>(reader: &mut R, ident: &ValueIdent) -> TychoResult<Value> {
    match ident {
        ValueIdent::Null => Ok(Value::Null),
        ValueIdent::Boolean => Ok(Value::Boolean(read_byte_async(reader).await? == 0x01)),
        ValueIdent::String => Ok(Value::String(read_string_async(reader).await?)),
        ValueIdent::Char => Ok(Value::Char(read_char_async(reader).await?)),
        ValueIdent::Number(n) => Ok(Value::Number(read_number_async(reader, n).await?)),
        ValueIdent::Bytes => {
            let length = read_length_async(reader).await?;
            Ok(Value::Bytes(read_bytes_async(reader, length).await?))
        }
        ValueIdent::UUID => {
            // suffering
            let bytes = [
                read_byte_async(reader).await?, read_byte_async(reader).await?,
                read_byte_async(reader).await?, read_byte_async(reader).await?,
                read_byte_async(reader).await?, read_byte_async(reader).await?,
                read_byte_async(reader).await?, read_byte_async(reader).await?,
                read_byte_async(reader).await?, read_byte_async(reader).await?,
                read_byte_async(reader).await?, read_byte_async(reader).await?,
                read_byte_async(reader).await?, read_byte_async(reader).await?,
                read_byte_async(reader).await?, read_byte_async(reader).await?,
            ];
            Ok(Value::UUID(Uuid::from_bytes(bytes)))
        }
    }
}