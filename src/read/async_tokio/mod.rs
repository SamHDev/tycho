pub mod func;
pub mod number;

use byteorder::ReadBytesExt;
use crate::error::{TychoResult, parse_io};
use tokio::io::{AsyncRead, AsyncReadExt};

pub(crate) async fn read_byte<R: AsyncRead + Unpin>(reader: &mut R) -> TychoResult<u8> {
    parse_io(reader.read_u8().await)
}

pub(crate) async fn read_bytes<R: AsyncRead + Unpin>(reader: &mut R, size: usize) -> TychoResult<Vec<u8>> {
    let mut buffer = Vec::with_capacity(size);
    for _ in 0..size {
        buffer.push(read_byte(reader).await?);
    }
    Ok(buffer)
}