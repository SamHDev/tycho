use tokio::io::{AsyncRead};
use crate::error::{TychoResult, parse_io};
use crate::types::ident::NumberIdent;
use crate::read::number::parse_number_ident;
use crate::read::async_tokio::func::read_byte_async;
use crate::Number;
use tokio_byteorder::AsyncReadBytesExt;
use byteorder::BE;

pub(crate) fn read_number_ident_async<R: AsyncRead>(reader: &mut R) -> TychoResult<NumberIdent> {
    parse_number_ident(read_byte_async(reader).await?)
}

pub(crate) fn read_number_async<R: AsyncRead>(reader: &mut R, ident: &NumberIdent) -> TychoResult<Number> {
    match ident {
        NumberIdent::Bit => Ok(Number::Bit(read_byte_async(reader)? == 0x00)),
        NumberIdent::Unsigned8 => Ok(Number::Unsigned8(parse_io(reader.read_u8().await)?)),
        NumberIdent::Signed8 => Ok(Number::Signed8(parse_io(reader.read_i8().await)?)),
        NumberIdent::Unsigned16 => Ok(Number::Unsigned16(parse_io(reader.read_u16::<BE>().await)?)),
        NumberIdent::Signed16 => Ok(Number::Signed16(parse_io(reader.read_i16::<BE>().await)?)),
        NumberIdent::Unsigned32 => Ok(Number::Unsigned32(parse_io(reader.read_u32::<BE>().await)?)),
        NumberIdent::Signed32 => Ok(Number::Signed32(parse_io(reader.read_i32::<BE>().await)?)),
        NumberIdent::Unsigned64 => Ok(Number::Unsigned64(parse_io(reader.read_u64::<BE>().await)?)),
        NumberIdent::Signed64 => Ok(Number::Signed64(parse_io(reader.read_i64::<BE>().await)?)),
        NumberIdent::Unsigned128 => Ok(Number::Unsigned128(parse_io(reader.read_u128::<BE>().await)?)),
        NumberIdent::Signed128 => Ok(Number::Signed128(parse_io(reader.read_i128::<BE>().await)?)),
        NumberIdent::Float32 => Ok(Number::Float32(parse_io(reader.read_f32::<BE>().await)?)),
        NumberIdent::Float64 => Ok(Number::Float64(parse_io(reader.read_f64::<BE>().await)?)),
    }
}
