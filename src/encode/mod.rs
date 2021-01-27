use crate::Element;
use crate::encode::element::ElementEncoder;

mod length;
pub(crate) mod element;
pub(crate) mod value;
mod util;
mod string;

pub fn encode(element: Element) -> Vec<u8> {
    element.encode()
}