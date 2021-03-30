//! Container types returned during partial parsing.

use std::io::{Read, Seek};

use crate::error::TychoResult;
use crate::partial::container::{PartialContainer, PartialContainerType};
use crate::partial::element::{PartialElement, read_partial_element};
use crate::partial::reader::PartialReader;
use crate::read::string::read_tstring;
use crate::read::value::read_value;
use crate::types::ident::ValueIdent;
use crate::Value;
use crate::partial::PartialPointer;
use crate::read::func::read_bytes;

#[derive(Debug, Clone)]
/// The inner implementation structure for a struct.
pub struct PartialStructInner;
impl PartialContainerType for PartialStructInner {
    type ItemType = (String, PartialElement);
    type ItemParam = ();

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, _: &()) -> TychoResult<Self::ItemType> {
        let key = read_tstring(reader)?;
        let value = read_partial_element(reader)?;
        Ok((key, value))
    }
}
/// A unprocessed struct object.
pub type PartialStruct = PartialContainer<PartialStructInner>;

#[derive(Debug, Clone)]
/// The inner implementation structure for a list.
pub struct PartialListInner;
impl PartialContainerType for PartialListInner {
    type ItemType = PartialElement;
    type ItemParam = ();

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, _: &()) -> TychoResult<Self::ItemType> {
        read_partial_element(reader)
    }
}
/// A unprocessed list object.
pub type PartialList = PartialContainer<PartialListInner>;

#[derive(Debug, Clone)]
/// The inner implementation structure for a map.
pub struct PartialMapInner;
impl PartialContainerType for PartialMapInner {
    type ItemType = (Value, PartialElement);
    type ItemParam = ValueIdent;

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, params: &ValueIdent) -> TychoResult<Self::ItemType> {
        let key = read_value(reader, &params)?;
        let value = read_partial_element(reader)?;
        Ok((key, value))
    }
}

/// A unprocessed map object.
pub type PartialMap = PartialContainer<PartialMapInner>;

#[derive(Debug, Clone)]
/// The inner implementation structure for a array.
pub struct PartialArrayInner;
impl PartialContainerType for PartialArrayInner {
    type ItemType = Value;
    type ItemParam = ValueIdent;

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, params: &ValueIdent) -> TychoResult<Self::ItemType> {
        let item = read_value(reader, &params)?;
        Ok(item)
    }
}
/// A unprocessed array object.
pub type PartialArray = PartialContainer<PartialArrayInner>;

#[derive(Debug, Clone)]
/// A unprocessed compression object.
pub struct PartialCompression {
    pub pointer: PartialPointer,
}

impl PartialCompression {
    pub(crate) fn new(pointer: PartialPointer) -> Self {
        PartialCompression { pointer }
    }

    /// Get the bytes within the compression object.
    pub fn bytes<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<Vec<u8>> {
        let top = reader.pointer.clone();
        reader.jump(&self.pointer.pos)?;
        let bytes = read_bytes(reader, self.pointer.size as usize)?;
        reader.jump(&top)?;
        Ok(bytes)
    }

    #[cfg(feature="compression")]
    /// Get the element within the compression object.
    ///
    /// (requires `compression` feature)
    pub fn element<R: Read + Seek>(&mut self, reader: &mut PartialReader<R>) -> TychoResult<PartialElement> {
        let top = reader.pointer.clone();
        reader.jump(&self.pointer.pos)?;
        let element = read_partial_element(reader)?;
        reader.jump(&top)?;
        Ok(element)
    }
}

#[cfg(feature = "async_tokio")]
pub use super::async_::types::PartialCompressionAsync;