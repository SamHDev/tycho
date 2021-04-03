use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, Error};
use serde::{Serialize, Serializer};

use crate::Element;
use crate::error::TychoError;
use crate::ident::ValueIdent;
use crate::into::ident::Ident;
use crate::serde::ser::TychoSerializer;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SeqSerializerType {
    Array(ValueIdent),
    List,
    Default
}

pub struct SeqSerializer {
    pub(crate) seq_type: SeqSerializerType,
    pub(crate) array_type: ValueIdent,
    pub(crate) array_opt: bool,
    pub(crate) elements: Vec<Element>
}

impl SeqSerializer {
    pub(crate) fn new(seq_type: SeqSerializerType) -> Self {
        Self {
            seq_type,
            array_type: ValueIdent::Null,
            array_opt: true,
            elements: Vec::new()
        }
    }

    pub fn element<T: ?Sized>(&mut self, value: &T) -> Result<(), TychoError> where
        T: Serialize {
        let value = value.serialize(TychoSerializer)?;

        #[cfg(feature = "serde_optimise")]
        if self.array_opt && self.seq_type == SeqSerializerType::Default {
            if let Element::Value(x) = &value {
                if self.array_type == ValueIdent::Null {
                    self.array_type = x.ident();
                } else if x.ident() != self.array_type {
                    self.array_opt = false;
                }
            } else {
                self.array_opt = false;
            }
        }

        self.elements.push(value);
        Ok(())
    }

    pub fn finish(self) -> Result<Element, TychoError> {
        match self.seq_type {
            SeqSerializerType::Default => {
                if self.elements.is_empty() {
                    Ok(Element::List(self.elements))
                } else {
                    #[cfg(feature = "serde_optimise")]
                    if self.array_opt {
                        return Ok(Element::Array(
                            self.array_type,
                            self.elements
                                .into_iter()
                                .filter_map(|x|
                                    if let Element::Value(v) = x { Some(v) } else { None })
                                .collect()
                        ));
                    } else {
                        return Ok(Element::List(self.elements));
                    }

                    #[cfg(not(feature = "serde_optimise"))]
                        return Ok(Element::List(self.elements));
                }
            }
            SeqSerializerType::Array(ident) => {
                Ok(Element::Array(ident.clone(),  self.elements
                    .into_iter()
                    .filter_map(|x|
                        if let Element::Value(v) = x { Some(v) } else { None })
                    .filter(|x| &x.ident() == &ident)
                    .collect()))
            },
            SeqSerializerType::List => {
                Ok(Element::List(self.elements))
            }
        }
    }
}


impl SerializeSeq for SeqSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
       self.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl SerializeTuple for SeqSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl SerializeTupleStruct for SeqSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}



impl Serializer for SeqSerializer {
    type Ok = Element;
    type Error = TychoError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = serde::ser::Impossible<Element, TychoError>;
    type SerializeMap = serde::ser::Impossible<Element, TychoError>;
    type SerializeStruct = serde::ser::Impossible<Element, TychoError>;
    type SerializeStructVariant = serde::ser::Impossible<Element, TychoError>;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: Serialize {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where
        T: fmt::Display {
        Err(TychoError::custom("called serialize on invalid serializer"))
    }
}