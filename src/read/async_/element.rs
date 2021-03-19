use std::collections::HashMap;
use std::io::Cursor;

use futures::future::{BoxFuture, FutureExt};
use tokio::io::AsyncRead;

use crate::Element;
use crate::error::TychoResult;
use crate::read::async_::func::{read_byte_async, read_bytes_async};
use crate::read::async_::length::read_length_async;
use crate::read::async_::string::read_tstring_async;
use crate::read::async_::value::{read_value_async, read_value_ident_async};
use crate::read::element::parse_element_ident;
use crate::types::ident::{ElementIdent, ValueIdent};

pub(crate) async fn read_element_ident_async<R: AsyncRead + Unpin>(reader: &mut R) -> TychoResult<ElementIdent> {
   parse_element_ident(read_byte_async(reader).await?)
}

pub(crate) fn read_element_async<'a, R: AsyncRead + Unpin + Send>(reader: &'a mut R) -> BoxFuture<'a, TychoResult<Element>> {
    async move {
        let ident = read_element_ident_async(reader).await?;

        match ident {
            ElementIdent::Unit => Ok(Element::Unit),
            ElementIdent::Value => {
                let ident = read_value_ident_async(reader).await?;
                Ok(Element::Value(read_value_async(reader, &ident).await?))
            },
            ElementIdent::Some => Ok(Element::Option(Some(Box::new(read_element_async(reader).await?)))),
            ElementIdent::None => Ok(Element::Option(None)),
            ElementIdent::Variant => Ok(Element::Variant(
                read_tstring_async(reader).await?,
                Box::new(read_element_async(reader).await?)
            )),
            ElementIdent::Struct => {
                let size = read_length_async(reader).await?;
                let mut items = HashMap::new();
                let mut buffer = Cursor::new(read_bytes_async(reader, size).await?);

                loop {
                    if buffer.position() == size as u64 { break; }

                    let key = read_tstring_async(&mut buffer).await?;
                    let value = read_element_async(&mut buffer).await?;

                    items.insert(key, value);
                }

                Ok(Element::Struct(items))
            }
            ElementIdent::List => {
                let size = read_length_async(reader).await?;
                let mut items = Vec::new();
                let mut buffer = Cursor::new(read_bytes_async(reader, size).await?);

                loop {
                    if buffer.position() == size as u64 { break; }
                    items.push(read_element_async(&mut buffer).await?);
                }

                Ok(Element::List(items))
            },
            ElementIdent::Array => {
                let array_type = read_value_ident_async(reader).await?;

                if let ValueIdent::Null = &array_type {
                    Ok(Element::Array(ValueIdent::Null, Vec::new()))
                } else {
                    let size = read_length_async(reader).await?;
                    let mut items = Vec::new();
                    let mut buffer = Cursor::new(read_bytes_async(reader, size).await?);

                    loop {
                        if buffer.position() == size as u64 { break; }
                        items.push(read_value_async(&mut buffer, &array_type).await?);
                    }

                    Ok(Element::Array(array_type, items))
                }
            },
            ElementIdent::Map => {
                let key_type = read_value_ident_async(reader).await?;

                if let ValueIdent::Null = &key_type {
                    Ok(Element::Map(ValueIdent::Null, HashMap::new()))
                } else {
                    let size = read_length_async(reader).await?;
                    let mut items = HashMap::new();
                    let mut buffer = Cursor::new(read_bytes_async(reader, size).await?);

                    loop {
                        if buffer.position() == size as u64 { break; }

                        let key = read_value_async(&mut buffer, &key_type).await?;
                        let value = read_element_async(&mut buffer).await?;

                        items.insert(key, value);
                    }

                    Ok(Element::Map(key_type, items))
                }
            },
            ElementIdent::Compression => {
                let size = read_length_async(reader).await?;
                let mut buffer = Cursor::new(read_bytes_async(reader, size).await?);
                Ok(Element::Compression(Box::new(read_element_async(&mut buffer).await?)))
            }
        }
    }.boxed()
}

