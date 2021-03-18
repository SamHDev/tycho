use crate::read::func::read_byte;
use crate::error::TychoResult;
use tokio::io::AsyncRead;
use crate::read::async_tokio::func::read_byte_async;

pub(crate) fn read_length_async<R: AsyncRead>(reader: &mut R) -> TychoResult<usize> {
    let mut number: u64 = 0;
    let mut count = 0;

    loop {
        let byte = read_byte_async(reader).await?;

        number |= ((byte & 0x7F) as u64) << (7 * count);

        if byte & 0x80 == 0 {
            return Ok(number as usize);
        }

        count += 1;
    }
}