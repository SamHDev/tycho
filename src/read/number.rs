use std::io::Read;

use byteorder::{BE, ReadBytesExt};

use crate::Number;
use crate::error::{parse_io, TychoError, TychoResult};
use crate::read::func::read_byte;
use crate::types::ident::NumberIdent;

pub(crate) fn read_number_ident<R: Read>(reader: &mut R) -> TychoResult<NumberIdent> {
    parse_number_ident(read_byte(reader)?)
}

pub(crate) fn parse_number_ident(byte: u8) -> TychoResult<NumberIdent> {
    match byte {
        0x00 => Ok(NumberIdent::Bit),
        0x01 => Ok(NumberIdent::Unsigned8),
        0x02 => Ok(NumberIdent::Unsigned16),
        0x03 => Ok(NumberIdent::Unsigned32),
        0x04 => Ok(NumberIdent::Unsigned64),
        0x05 => Ok(NumberIdent::Unsigned128),
        0x11 => Ok(NumberIdent::Signed8),
        0x12 => Ok(NumberIdent::Signed16),
        0x13 => Ok(NumberIdent::Signed32),
        0x14 => Ok(NumberIdent::Signed64),
        0x15 => Ok(NumberIdent::Signed128),
        0x23 => Ok(NumberIdent::Float32),
        0x24 => Ok(NumberIdent::Float64),

        _ => Err(TychoError::InvalidIdent { found: byte, expecting: "number ident".to_string() })
    }
}

pub(crate) fn read_number<R: Read>(reader: &mut R, ident: &NumberIdent) -> TychoResult<Number> {
    match ident {
        NumberIdent::Bit => Ok(Number::Bit(read_byte(reader)? == 0x00)),
        NumberIdent::Unsigned8 => Ok(Number::Unsigned8(parse_io(reader.read_u8())?)),
        NumberIdent::Signed8 => Ok(Number::Signed8(parse_io(reader.read_i8())?)),
        NumberIdent::Unsigned16 => Ok(Number::Unsigned16(parse_io(reader.read_u16::<BE>())?)),
        NumberIdent::Signed16 => Ok(Number::Signed16(parse_io(reader.read_i16::<BE>())?)),
        NumberIdent::Unsigned32 => Ok(Number::Unsigned32(parse_io(reader.read_u32::<BE>())?)),
        NumberIdent::Signed32 => Ok(Number::Signed32(parse_io(reader.read_i32::<BE>())?)),
        NumberIdent::Unsigned64 => Ok(Number::Unsigned64(parse_io(reader.read_u64::<BE>())?)),
        NumberIdent::Signed64 => Ok(Number::Signed64(parse_io(reader.read_i64::<BE>())?)),
        NumberIdent::Unsigned128 => Ok(Number::Unsigned128(parse_io(reader.read_u128::<BE>())?)),
        NumberIdent::Signed128 => Ok(Number::Signed128(parse_io(reader.read_i128::<BE>())?)),
        NumberIdent::Float32 => Ok(Number::Float32(parse_io(reader.read_f32::<BE>())?)),
        NumberIdent::Float64 => Ok(Number::Float64(parse_io(reader.read_f64::<BE>())?)),
    }
}
