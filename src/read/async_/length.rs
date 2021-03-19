use tokio::io::AsyncRead;

use crate::error::TychoResult;
use crate::read::async_::func::read_byte_async;

pub(crate) async fn read_length_async<R: AsyncRead + Unpin>(reader: &mut R) -> TychoResult<usize> {
    let mut number: u64 = 0;
    let mut count = 0_i32;

    loop {
        let byte = read_byte_async(reader).await?;

        number |= ((byte & 0x7F) as u64) << (7 * count);

        if byte & 0x80 == 0 {
            return Ok(number as usize);
        }

        count += 1;
    }
}