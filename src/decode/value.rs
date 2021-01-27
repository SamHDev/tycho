use crate::ident::ValueIdent;
use crate::decode::reader::Reader;
use crate::decode::error::DecodeError;
use crate::Value;
use crate::decode::string::{read_string, read_term_string};
use crate::decode::length::read_var_length;

pub(crate) fn decode_value(value_type: ValueIdent, reader: &mut Reader ) -> Result<Value, DecodeError> {
    match value_type {
        ValueIdent::Boolean => Ok(Value::Boolean(reader.read_one()? == 0x01)),
        ValueIdent::Unsigned8 => Ok(Value::Unsigned8(reader.read_u8()?)),
        ValueIdent::Unsigned16 => Ok(Value::Unsigned16(reader.read_u16()?)),
        ValueIdent::Unsigned32 => Ok(Value::Unsigned32(reader.read_u32()?)),
        ValueIdent::Unsigned64 => Ok(Value::Unsigned64(reader.read_u64()?)),
        ValueIdent::Unsigned128 => Ok(Value::Unsigned128(reader.read_u128()?)),
        ValueIdent::Signed8 => Ok(Value::Signed8(reader.read_i8()?)),
        ValueIdent::Signed16 => Ok(Value::Signed16(reader.read_i16()?)),
        ValueIdent::Signed32 => Ok(Value::Signed32(reader.read_i32()?)),
        ValueIdent::Signed64 => Ok(Value::Signed64(reader.read_i64()?)),
        ValueIdent::Signed128 => Ok(Value::Signed128(reader.read_i128()?)),
        ValueIdent::Float32 => Ok(Value::Float32(reader.read_f32()?)),
        ValueIdent::Float64 => Ok(Value::Float64(reader.read_f64()?)),
        ValueIdent::String => Ok(Value::String(read_string(reader)?)),
        ValueIdent::Char => match read_term_string(reader)?.chars().nth(0) {
            Some(c) => Ok(Value::Char(c)),
            None => Err(DecodeError::BadCharLength { pos: reader.pos()})
        }
        ValueIdent::Bytes => {
            let length = read_var_length(reader)?;
            Ok(Value::Bytes(reader.read_many(length as usize)?))
        }
        ValueIdent::NIL => Err(DecodeError::LogicalDecodeError { pos: reader.pos() })
    }
}
