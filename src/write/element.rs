use std::io::{BufWriter, Write};

use crate::Element;
use crate::error::TychoStatus;
use crate::into::ident::Ident;
use crate::types::ident::ValueIdent;
use crate::write::func::{write_buffer, write_byte};
use crate::write::string::write_tstring;
use crate::write::value::{write_value, write_value_ident};

pub(crate) fn write_element<W: Write>(writer: &mut W, element: &Element) -> TychoStatus {
    match element {
        Element::Unit => write_byte(writer, &0x00),
        Element::Value(value) => {
            write_byte(writer, &0x01)?;
            write_value_ident(writer, &value.ident())?;
            write_value(writer, &value)
        },
        Element::Option(opt) => match opt {
            None => write_byte(writer, &0x02),
            Some(x) => {
                write_byte(writer, &0x03)?;
                write_element(writer, x)
            }
        }
        Element::Variant(name, element) => {
            write_byte(writer, &0x04)?;
            write_tstring(writer, name)?;
            write_element(writer, element)
        }
        Element::Struct(data) => {
            write_byte(writer, &0x05)?;
            let mut buffer = BufWriter::new(Vec::new());
            for (key, value) in data {
                write_tstring(&mut buffer, key)?;
                write_element(&mut buffer, value)?;
            }
            write_buffer(writer, buffer)
        }
        Element::List(data) => {
            write_byte(writer, &0x06)?;
            let mut buffer = BufWriter::new(Vec::new());
            for item in data {
                write_element(&mut buffer, item)?;
            }
            write_buffer(writer, buffer)
        }
        Element::Array(array_type, data) => {
            write_byte(writer, &0x07)?;
            if array_type == &ValueIdent::Null || data.is_empty() {
                write_value_ident(writer, &ValueIdent::Null)
            } else {
                let mut buffer = BufWriter::new(Vec::new());
                for item in data {
                    write_value(&mut buffer, &item)?;
                }
                write_value_ident(writer, array_type)?;
                write_buffer(writer, buffer)
            }
        }
        Element::Map(key_type, data) => {
            write_byte(writer, &0x08)?;
            if key_type == &ValueIdent::Null || data.is_empty() {
                write_value_ident(writer, &ValueIdent::Null)
            } else {
               let mut buffer = BufWriter::new(Vec::new());
               for (key, value) in data {
                   write_value(&mut buffer, &key)?;
                   write_element(&mut buffer, &value)?;
               }
               write_value_ident(writer, key_type)?;
               write_buffer(writer, buffer)
           }
        }
        Element::Compression(compression) => {
            write_byte(writer, &0xF0)?;
            let mut buffer = BufWriter::new(Vec::new());
            write_element(&mut buffer, compression)?;
            // todo: compression
            write_buffer(writer, buffer)
        }
    }
}