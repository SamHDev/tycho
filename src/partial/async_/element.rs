use tokio::io::{AsyncRead, AsyncSeek};

use crate::error::TychoResult;
use crate::ident::ElementIdent;
use crate::partial::{PartialElement, PartialMap, PartialReader, PartialStruct};
use crate::partial::{PartialArray, PartialList};
use crate::read::async_::element::read_element_ident_async;
use crate::read::async_::length::read_length_async;
use crate::read::async_::string::read_tstring_async;
use crate::read::async_::value::{read_value_async, read_value_ident_async};
use crate::types::ident::ValueIdent;
use futures::future::BoxFuture;
use futures::FutureExt;

pub fn read_partial_element_async<R: AsyncRead + AsyncSeek + Unpin + Send>(reader: &mut PartialReader<R>) -> BoxFuture<TychoResult<PartialElement>> {
    async move {
        let ident = read_element_ident_async(reader).await?;

        match ident {
            ElementIdent::Unit => Ok(PartialElement::Unit),
            ElementIdent::Value => {
                let prefix = read_value_ident_async(reader).await?;
                let value = read_value_async(reader, &prefix).await?;
                Ok(PartialElement::Value(value))
            },

            ElementIdent::None => Ok(PartialElement::Option(None)),
            ElementIdent::Some => read_partial_element_async(reader).await,

            ElementIdent::Variant => {
                let name = read_tstring_async(reader).await?;
                let value = read_partial_element_async(reader).await?;

                Ok(PartialElement::Variant(name, Box::new(value)))
            }

            ElementIdent::Struct => {
                let size = read_length_async(reader).await? as u64;
                let pos = reader.pointer.clone();
                reader.jump_async(&size).await?;
                Ok(PartialElement::Struct(PartialStruct::new(reader.pointer(pos, size), 0, ())))
            },

            ElementIdent::List => {
                let size = read_length_async(reader).await? as u64;
                let pos = reader.pointer.clone();
                reader.jump_async(&size).await?;
                Ok(PartialElement::List(PartialList::new(reader.pointer(pos, size), 0, ())))
            },

            ElementIdent::Array => {
                let array_type = read_value_ident_async(reader).await?;

                if array_type == ValueIdent::Null {
                    return Ok(PartialElement::Array(PartialArray::empty(reader.empty_pointer(), array_type)))
                }

                let size = read_length_async(reader).await? as u64;
                let pos = reader.pointer.clone();
                reader.jump_async(&size).await?;
                Ok(PartialElement::Array(PartialArray::new(reader.pointer(pos, size), 0, array_type)))
            },

            ElementIdent::Map => {
                let key_type = read_value_ident_async(reader).await?;

                if key_type == ValueIdent::Null {
                    return Ok(PartialElement::Map(PartialMap::empty(reader.empty_pointer(), key_type)))
                }

                let size = read_length_async(reader).await? as u64;
                let pos = reader.pointer.clone();
                reader.jump_async(&size).await?;
                Ok(PartialElement::Map(PartialMap::new(reader.pointer(pos, size), 0, key_type)))
            },

            _ => { panic!("{:?}", ident) }
        }
    }.boxed()
}