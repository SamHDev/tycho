use std::io::Write;
use crate::def::ident::{Ident, ElementIdent, NumberIdent, ValueIdent};
use crate::error::TychoStatus;
use byteorder::WriteBytesExt;
use std::process::id;

pub(crate) static NUM_FLOAT: u8 = 0xF0;
pub(crate) static NUM_SIGNED: u8 = 0x70;
pub(crate) static NUM_LEN_8: u8 = 0x00;
pub(crate) static NUM_LEN_16: u8 = 0x01;
pub(crate) static NUM_LEN_32: u8 = 0x02;
pub(crate) static NUM_LEN_64: u8 = 0x03;
pub(crate) static NUM_LEN_128: u8 = 0x04;


pub(crate) fn write_number_ident<W: Write>(writer: &mut W, ident: &NumberIdent) -> TychoStatus {
    TychoStatus::digest_io(writer.write_u8(match &ident {
        NumberIdent::Unsigned8 => NUM_LEN_8,
        NumberIdent::Unsigned16 => NUM_LEN_16,
        NumberIdent::Unsigned32 => NUM_LEN_32,
        NumberIdent::Unsigned64 => NUM_LEN_64,
        NumberIdent::Unsigned128 => NUM_LEN_128,
        NumberIdent::Signed8 => NUM_LEN_8 + NUM_SIGNED,
        NumberIdent::Signed16 => NUM_LEN_16 + NUM_SIGNED,
        NumberIdent::Signed32 => NUM_LEN_32 + NUM_SIGNED,
        NumberIdent::Signed64 => NUM_LEN_64 + NUM_SIGNED,
        NumberIdent::Signed128 => NUM_LEN_128 + NUM_SIGNED,
        NumberIdent::Float32 => NUM_FLOAT + NUM_LEN_32,
        NumberIdent::Float64 => NUM_FLOAT + NUM_LEN_64
    }))
}

pub(crate) fn write_value_ident<W: Write>(writer: &mut W, ident: &ValueIdent) -> TychoStatus {
    match &ident {
        ValueIdent::Boolean => TychoStatus::digest_io(writer.write_u8(0x01)),
        ValueIdent::Number(num) => {
            TychoStatus::digest_io(writer.write_u8(0x02))?;
            write_number_ident(writer, num)
        }
        ValueIdent::String => TychoStatus::digest_io(writer.write_u8(0x03)),
        ValueIdent::Char => TychoStatus::digest_io(writer.write_u8(0x04)),
        ValueIdent::Bytes => TychoStatus::digest_io(writer.write_u8(0x05)),
        ValueIdent::Uuid => TychoStatus::digest_io(writer.write_u8(0x06)),
    }
}

