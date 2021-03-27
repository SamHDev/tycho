use std::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncSeek, AsyncSeekExt, ReadBuf, SeekFrom};

use crate::error::{parse_io, TychoResult, TychoStatus};
use crate::partial::{PartialElement, PartialReader};
use crate::partial::async_::element::read_partial_element_async;
use std::pin::Pin;

#[allow(type_alias_bounds)]
pub type PartialAsyncReader<R: AsyncSeek + AsyncRead + Unpin> = PartialReader<R>;

impl<R: AsyncSeek + AsyncRead + Unpin + Send> PartialReader<R> {
    pub async fn jump_async(&mut self, to: &u64) -> TychoStatus {
        parse_io(self.reader.seek(SeekFrom::Current((*to as i64) - (self.pointer as i64))).await)?;
        self.pointer = *to;
        Ok(())
    }

    pub async fn element_async(&mut self) -> TychoResult<PartialElement> {
        read_partial_element_async(self).await
    }
}

impl<R: AsyncRead + AsyncSeek + Unpin> AsyncRead for PartialReader<R> {
    fn poll_read(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<std::io::Result<()>> {
        match AsyncRead::poll_read(Pin::new(&mut self), cx, buf) {
            Poll::Ready(x) => match x {
                Ok(_) => {
                    self.pointer += buf.filled().len() as u64;
                    Poll::Ready(Ok(()))
                }
                Err(e) => Poll::Ready(Err(e))
            }
            Poll::Pending => Poll::Pending
        }
    }
}
