use std::io::Write;
use byteorder::{WriteBytesExt, BE};

use crate::{Number};
use crate::write::func::write_byte;
use crate::types::ident::NumberIdent;
use crate::error::{TychoStatus, parse_io};

pub(crate) const NUM_LEN_1: u8 = 0x00;
pub(crate) const NUM_LEN_8: u8 = 0x01;
pub(crate) const NUM_LEN_16: u8 = 0x02;
pub(crate) const NUM_LEN_32: u8 = 0x03;
pub(crate) const NUM_LEN_64: u8 = 0x04;
pub(crate) const NUM_LEN_128: u8 = 0x05;
pub(crate) const NUM_FLOAT: u8 = 0x20;
pub(crate) const NUM_SIGNED: u8 = 0x10;

pub(crate) fn write_number_ident<W: Write>(writer: &mut W, ident: &NumberIdent) -> TychoStatus {
    let value = match ident {
        NumberIdent::Bit => NUM_LEN_1,
        NumberIdent::Unsigned8 => NUM_LEN_8,
        NumberIdent::Signed8 => NUM_LEN_8 | NUM_SIGNED,
        NumberIdent::Unsigned16 => NUM_LEN_16,
        NumberIdent::Signed16 => NUM_LEN_16 | NUM_SIGNED,
        NumberIdent::Unsigned32 => NUM_LEN_32,
        NumberIdent::Signed32 => NUM_LEN_32 | NUM_SIGNED,
        NumberIdent::Unsigned64 => NUM_LEN_64,
        NumberIdent::Signed64 => NUM_LEN_64 | NUM_SIGNED,
        NumberIdent::Unsigned128 => NUM_LEN_128,
        NumberIdent::Signed128 => NUM_LEN_128 | NUM_SIGNED,
        NumberIdent::Float32 => NUM_LEN_32 | NUM_FLOAT,
        NumberIdent::Float64 => NUM_LEN_64 | NUM_FLOAT,
    };
    write_byte(writer, &value)
}

pub(crate) fn write_number<W: Write>(writer: &mut W, number: &Number) -> TychoStatus {
    match number {
        Number::Bit(x) => match x {
            true => write_byte(writer, &0x01),
            false => write_byte(writer, &0x00),
        },
        Number::Unsigned8(x) => parse_io(writer.write_u8(*x)),
        Number::Signed8(x) =>  parse_io(writer.write_i8(*x)),
        Number::Unsigned16(x) => parse_io(writer.write_u16::<BE>(*x)),
        Number::Signed16(x) => parse_io(writer.write_i16::<BE>(*x)),
        Number::Unsigned32(x) => parse_io(writer.write_u32::<BE>(*x)),
        Number::Signed32(x) => parse_io(writer.write_i32::<BE>(*x)),
        Number::Unsigned64(x) => parse_io(writer.write_u64::<BE>(*x)),
        Number::Signed64(x) => parse_io(writer.write_i64::<BE>(*x)),
        Number::Unsigned128(x) => parse_io(writer.write_u128::<BE>(*x)),
        Number::Signed128(x) => parse_io(writer.write_i128::<BE>(*x)),
        Number::Float32(x) => parse_io(writer.write_f32::<BE>(*x)),
        Number::Float64(x) => parse_io(writer.write_f64::<BE>(*x)),
    }
}