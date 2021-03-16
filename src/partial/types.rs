use crate::partial::container::{PartialContainerType, PartialContainer};
use crate::partial::element::PartialElement;
use crate::error::TychoResult;
use crate::partial::reader::PartialReader;
use std::io::{Read, Seek};
use crate::read::string::read_tstring;

#[derive(Debug, Clone)]
pub struct PartialStructInner;
impl PartialContainerType for PartialStructInner {
    type ItemType = (String, PartialElement);

    fn read_item<R: Read + Seek>(reader: &mut PartialReader<R>) -> TychoResult<Self::ItemType> {
        let key = read_tstring(reader)?;
        let value = PartialElement::read(reader)?;
        Ok((key, value))
    }
}
pub type PartialStruct = PartialContainer<PartialStructInner>;