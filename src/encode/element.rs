use crate::def::elements::Element;
use std::io::Write;
use crate::error::TychoStatus;
use byteorder::WriteBytesExt;
use crate::encode::ident::write_value_ident;
use crate::def::ident::Ident;

pub(crate) fn write_element<W: Write>(writer: &mut W, ident: &Element) -> TychoStatus {
    match ident {
        Element::Unit => TychoStatus::digest_io(writer.write_u8(0x00)),
        Element::Value(value) => {
            TychoStatus::digest_io(writer.write_u8(0x01))?;
            write_value_ident(writer, &value.ident())?;

        }
        Element::Option(data) => match data {
            Some(value) => TychoStatus::digest_io(writer.write_all(&[0x12, 0x00])),
        }
        Element::Struct(_) => {}
        Element::List(_) => {}
        Element::Map(_) => {}
        Element::Array(_) => {}
    }
}