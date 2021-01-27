pub(crate) mod value;
pub(crate) mod reader;
pub(crate) mod error;
pub(crate) mod string;
pub(crate) mod length;
pub(crate) mod element;

use crate::Element;
pub use crate::decode::error::DecodeError;
use crate::decode::element::decode_prefixed_element;
use crate::decode::reader::Reader;

pub fn decode(data: Vec<u8>) -> Result<Element, DecodeError> {
    decode_prefixed_element(&mut Reader::create(data))
}