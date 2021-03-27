use crate::error::TychoResult;
use crate::partial::reader::PartialReader;
use async_trait::async_trait;
use tokio::io::{AsyncSeek, AsyncRead};
use crate::partial::async_::container::PartialContainerTypeAsync;
use crate::read::async_::string::read_tstring_async;
use crate::partial::async_::element::read_partial_element_async;
use crate::partial::types::{PartialStructInner, PartialListInner, PartialMapInner, PartialArrayInner};
use crate::read::async_::value::read_value_async;

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