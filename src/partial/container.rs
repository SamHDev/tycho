use std::io::{Read, Seek};
use std::marker::PhantomData;

use crate::error::{TychoError, TychoResult};
use crate::partial::reader::{PartialPointer, PartialReader};

pub trait PartialContainerType {
    type ItemType;
    type ItemParam;
    //type ItemStandard;

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, param: &Self::ItemParam) -> TychoResult<Self::ItemType>;
    //fn standardise(items: Vec<T::ItemType>) -> TychoResult<Self::ItemStandard>;
}

#[derive(Debug, Clone)]
pub struct PartialContainer<T: PartialContainerType> {
    /// The container start pointer.
    pub pointer: PartialPointer,
    /// The local container head pointer.
    pub head: u64,
    /// Parameters for the container.
    pub param: T::ItemParam,
    _phantom: PhantomData<T>
}

impl<T: PartialContainerType> PartialContainer<T> {
    /// Create a new partial container
    pub(crate) fn new(pointer: PartialPointer, head: u64, param: T::ItemParam) -> Self {
        PartialContainer { pointer, head, _phantom: Default::default(), param }
    }

    /// Create an empty partial container
    pub(crate) fn empty(pointer: PartialPointer, param: T::ItemParam) -> Self {
        PartialContainer { pointer, head: 0, _phantom: Default::default(), param }
    }

    /// Read next item
    pub(crate) fn next_item<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Option<T::ItemType>> {
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
        reader.jump(&(self.pointer.pos + self.head))?;
        let head_start = reader.pointer;

        let item = T::read_item(reader, &self.param)?;

        // increment head
        self.head += reader.pointer - head_start;

        // reset pointer
        reader.jump(&top)?;

        // return item
        Ok(Some(item))
    }

    /// Returns if the local pointer head has reached the end of the container.
    pub fn finished(&self) -> bool { self.head == self.pointer.size }

    /// Get the next item in the container
    pub fn next<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Option<T::ItemType>> {
        self.next_item(reader)
    }

    /// Get an iterator of the container.
    pub fn iter<'x, R: Read + Seek>(&'x mut self, reader: &'x mut PartialReader<R>) -> PartialContainerIterator<'x, T, R> {
        PartialContainerIterator::new(self, reader)
    }

    /// Collect all items within the container.
    pub fn collect<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Vec<T::ItemType>> {
        let mut items = Vec::new();
        while let Some(item) = self.next_item(reader)? {
            items.push(item);
        }
        Ok(items)
    }

    /// Move the head to the top/start
    pub fn top(&mut self) {
        self.head = 0;
    }

    /*pub fn standardise<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<T::ItemStandard> {
        let head = self.head;
        self.head = 0;

        let items = self.collect(reader)?;

        let standard = T::standardise(items)?;

        self.head = head;

        Ok(standard)
    }

    pub fn replace<R: Read + Seek + Write>(&mut self, reader: &mut PartialReader<R>, element: T::) {
        reader.jump(&self.pointer.pos);
        reader.reader.write_all(&element)
    }*/
}

pub struct PartialContainerIterator<'x, T: PartialContainerType, R: Read + Seek>(
    &'x mut PartialContainer<T>,
    &'x mut PartialReader<R>
);

impl<'x, T: PartialContainerType, R: Read + Seek> PartialContainerIterator<'x, T, R> {
    pub fn new(
        container: &'x mut PartialContainer<T>,
        reader: &'x mut PartialReader<R>
    ) -> PartialContainerIterator<'x, T, R> {
        Self {
            0: container,
            1: reader
        }
    }
}

impl<'x, T: PartialContainerType, R: Read + Seek> Iterator for PartialContainerIterator<'x, T, R> {
    type Item = T::ItemType;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next_item(self.1).ok()?
    }
}