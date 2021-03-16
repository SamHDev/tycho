use std::io::{Read, Seek};
use crate::partial::reader::{PartialReader, PartialPointer};
use crate::error::TychoResult;
use std::marker::PhantomData;

pub trait PartialContainerType {
    type ItemType;

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>) -> TychoResult<Self::ItemType>;
}

#[derive(Debug, Clone)]
pub struct PartialContainer<T: PartialContainerType> {
    pub pointer: PartialPointer,
    pub head: u64,
    _phantom: PhantomData<T>
}

impl<T: PartialContainerType> PartialContainer<T> {
    pub(crate) fn new(pos: u64, size: u64, head: u64) -> Self {
        PartialContainer { pointer: PartialPointer { pos, size }, head, _phantom: Default::default() }
    }

    pub(crate) fn next_item<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Option<T::ItemType>> {
        // Check that the list is not finished
        if self.head == self.pointer.size {
            return Ok(None);
        }

        // get current pointer pos
        let top = reader.pointer.clone();

        // jump to next item
        reader.jump(&(self.pointer.pos + self.head))?;
        let head_start = reader.pointer;

        let item = T::read_item(reader)?;

        // increment head
        self.head += reader.pointer - head_start;

        // reset pointer
        reader.jump(&top)?;

        // return item
        Ok(Some(item))
    }

    pub fn finished(&self) -> bool { self.head == self.pointer.size }

    pub fn next<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Option<T::ItemType>> {
        self.next_item(reader)
    }

    pub fn iter<'x, R: Read + Seek>(&'x mut self, reader: &'x mut PartialReader<R>) -> PartialContainerIterator<'x, T, R> {
        PartialContainerIterator::new(self, reader)
    }
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