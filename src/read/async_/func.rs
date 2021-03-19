use tokio::io::AsyncRead;
use tokio_byteorder::AsyncReadBytesExt;

use crate::error::{parse_io, TychoResult};

pub(crate) async fn read_byte_async<R: AsyncRead + Unpin>(reader: &mut R) -> TychoResult<u8> {
    parse_io(reader.read_u8().await)
}

pub(crate) async fn read_bytes_async<R: AsyncRead + Unpin>(reader: &mut R, size: usize) -> TychoResult<Vec<u8>> {
    let mut buffer = Vec::with_capacity(size);
    for _ in 0..size {
        buffer.push(read_byte_async(reader).await?);
    }
    Ok(buffer)
}