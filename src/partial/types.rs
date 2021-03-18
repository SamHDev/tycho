use std::io::{Read, Seek};

use crate::error::TychoResult;
use crate::partial::container::{PartialContainer, PartialContainerType};
use crate::partial::element::PartialElement;
use crate::partial::reader::PartialReader;
use crate::read::string::read_tstring;
use crate::read::value::read_value;
use crate::types::ident::ValueIdent;
use crate::Value;

#[derive(Debug, Clone)]
pub struct PartialStructInner;
impl PartialContainerType for PartialStructInner {
    type ItemType = (String, PartialElement);
    type ItemParam = ();

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, _: &()) -> TychoResult<Self::ItemType> {
        let key = read_tstring(reader)?;
        let value = PartialElement::read(reader)?;
        Ok((key, value))
    }
}
pub type PartialStruct = PartialContainer<PartialStructInner>;

#[derive(Debug, Clone)]
pub struct PartialListInner;
impl PartialContainerType for PartialListInner {
    type ItemType = PartialElement;
    type ItemParam = ();

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, _: &()) -> TychoResult<Self::ItemType> {
        PartialElement::read(reader)
    }
}
pub type PartialList = PartialContainer<PartialListInner>;

#[derive(Debug, Clone)]
pub struct PartialMapInner;
impl PartialContainerType for PartialMapInner {
    type ItemType = (Value, PartialElement);
    type ItemParam = ValueIdent;

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, params: &ValueIdent) -> TychoResult<Self::ItemType> {
        let key = read_value(reader, &params)?;
        let value = PartialElement::read(reader)?;
        Ok((key, value))
    }
}
pub type PartialMap = PartialContainer<PartialMapInner>;

#[derive(Debug, Clone)]
pub struct PartialArrayInner;
impl PartialContainerType for PartialArrayInner {
    type ItemType = Value;
    type ItemParam = ValueIdent;

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>, params: &ValueIdent) -> TychoResult<Self::ItemType> {
        let item = read_value(reader, &params)?;
        Ok(item)
    }
}
pub type PartialArray = PartialContainer<PartialArrayInner>;