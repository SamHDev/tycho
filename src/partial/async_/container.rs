use crate::partial::container::{PartialContainerType, PartialContainer};
use tokio::io::{AsyncRead, AsyncSeek};
use crate::partial::PartialReader;
use crate::error::{TychoResult, TychoError};
use async_trait::async_trait;

#[async_trait]
pub trait PartialContainerTypeAsync: PartialContainerType {
    async fn read_item_async<R: AsyncRead + AsyncSeek + Unpin + Send>(reader: &mut PartialReader<R>, params: &Self::ItemParam) -> TychoResult<Self::ItemType>;
}

impl<T: PartialContainerTypeAsync> PartialContainer<T> {

    pub(crate) async fn next_item_async<R: AsyncRead + AsyncSeek + Unpin + Send>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Option<T::ItemType>> {
        #[cfg(feature="partial_state")]
        if self.pointer.ident != reader.ident {
            return Err(TychoError::OutdatedPointer)
        }

        // Check that the list is not finished
        if self.head == self.pointer.size {
            return Ok(None);
        }

        // get current pointer pos
        let top = reader.pointer.clone();

        // jump to next item
        reader.jump_async(&(self.pointer.pos + self.head)).await?;
        let head_start = reader.pointer;

        let item = T::read_item_async(reader, &self.param).await?;

        // increment head
        self.head += reader.pointer - head_start;

        // reset pointer
        reader.jump_async(&top).await?;

        // return item
        Ok(Some(item))
    }

    pub async fn next_async<R: AsyncRead + AsyncSeek + Unpin + Send>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Option<T::ItemType>> {
        self.next_item_async(reader).await
    }

    /*pub fn iter_async<'x, R: AsyncRead + AsyncSeek + Unpin + Send>(&'x mut self, reader: &'x mut PartialReader<R>) -> PartialContainerAsyncIterator<'x, T, R> {
        PartialContainerAsyncIterator::new(self, reader)
    }*/

    pub async fn collect_async<R: AsyncRead + AsyncSeek + Unpin + Send>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Vec<T::ItemType>> {
        let mut items = Vec::new();
        while let Some(item) = self.next_item_async(reader).await? {
            items.push(item);
        }
        Ok(items)
    }
}


/*
pub struct PartialContainerAsyncIterator<'x, T: PartialContainerTypeAsync, R: AsyncRead + AsyncSeek + Unpin + Send> (
    &'x mut PartialContainer<T>,
    &'x mut PartialReader<R>
);

impl<'x, T: PartialContainerTypeAsync, R: AsyncRead + AsyncSeek + Unpin + Send> PartialContainerAsyncIterator<'x, T, R> {
    pub fn new(
        container: &'x mut PartialContainer<T>,
        reader: &'x mut PartialReader<R>
    ) -> PartialContainerAsyncIterator<'x, T, R> {
        Self {
            0: container,
            1: reader
        }
    }

    pub(crate) async fn handle_next(&mut self) -> Option<T::ItemType> {
        self.0.next_async(&mut self.0).await.ok()?
    }
}

impl<'x, T: PartialContainerTypeAsync, R: AsyncRead + AsyncSeek + Unpin + Send> Iterator for PartialContainerAsyncIterator<'x, T, R> {
    type Item = Box<dyn Future<Output=Option<T::ItemType>>>;

    fn next(&mut self) -> Self::Item {
        Box::new(self.handle_next())
    }
}

*/