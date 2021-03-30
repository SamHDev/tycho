use crate::error::TychoResult;
use crate::partial::reader::PartialReader;
use async_trait::async_trait;
use tokio::io::{AsyncSeek, AsyncRead};
use crate::partial::async_::container::PartialContainerTypeAsync;
use crate::read::async_::string::read_tstring_async;
use crate::partial::async_::element::read_partial_element_async;
use crate::partial::types::{PartialStructInner, PartialListInner, PartialMapInner, PartialArrayInner, PartialCompression};
use crate::read::async_::value::read_value_async;
use crate::read::async_::func::read_bytes_async;
use crate::partial::{PartialElement};

#[async_trait]
impl PartialContainerTypeAsync for PartialStructInner {
    async fn read_item_async<R: AsyncRead + AsyncSeek + Unpin + Send>(reader: &mut PartialReader<R>, _params: &Self::ItemParam) -> TychoResult<Self::ItemType> {
        let key = read_tstring_async(reader).await?;
        let value = read_partial_element_async(reader).await?;
        Ok((key, value))
    }
}

#[async_trait]
impl PartialContainerTypeAsync for PartialListInner {
    async fn read_item_async<R: AsyncRead + AsyncSeek + Unpin + Send>(reader: &mut PartialReader<R>, _params: &Self::ItemParam) -> TychoResult<Self::ItemType> {
        read_partial_element_async(reader).await
    }
}

#[async_trait]
impl PartialContainerTypeAsync for PartialMapInner {
    async fn read_item_async<R: AsyncRead + AsyncSeek + Unpin + Send>(reader: &mut PartialReader<R>, params: &Self::ItemParam) -> TychoResult<Self::ItemType> {
        let key = read_value_async(reader, &params).await?;
        let value = read_partial_element_async(reader).await?;
        Ok((key, value))
    }
}

#[async_trait]
impl PartialContainerTypeAsync for PartialArrayInner {
    async fn read_item_async<R: AsyncRead + AsyncSeek + Unpin + Send>(reader: &mut PartialReader<R>, params: &Self::ItemParam) -> TychoResult<Self::ItemType> {
        read_value_async(reader, &params).await
    }
}

#[async_trait]
/// Async implementations for `PartialCompression`
pub trait PartialCompressionAsync {
    async fn bytes_async<R: AsyncRead + AsyncSeek + Unpin + Send>(&self, reader: &mut PartialReader<R>) -> TychoResult<Vec<u8>>;
    #[cfg(feature="compression")]
    async fn element_async<R: AsyncRead + AsyncSeek + Unpin + Send>(&self, reader: &mut PartialReader<R>) -> TychoResult<PartialElement>;
}

#[async_trait]
impl PartialCompressionAsync for PartialCompression {
    /// Get the bytes within the compression object asynchronously.
    ///
    /// (requires  `async_tokio` feature)
    async fn bytes_async<R: AsyncRead + AsyncSeek + Unpin + Send>(&self, reader: &mut PartialReader<R>) -> TychoResult<Vec<u8>> {
        let top = reader.pointer.clone();
        reader.jump_async(&self.pointer.pos).await?;
        let bytes = read_bytes_async(reader, self.pointer.size as usize).await?;
        reader.jump_async(&top).await?;
        Ok(bytes)
    }

    #[cfg(feature="compression")]
    /// Get the element within the compression object asynchronously.
    ///
    /// (requires `compression` feature and `async_tokio` feature)
    async fn element_async<R: AsyncRead + AsyncSeek + Unpin + Send>(&self, reader: &mut PartialReader<R>) -> TychoResult<PartialElement> {
        let top = reader.pointer.clone();
        reader.jump_async(&self.pointer.pos).await?;
        let element = read_partial_element_async(reader).await?;
        reader.jump_async(&top).await?;
        Ok(element)
    }
}
