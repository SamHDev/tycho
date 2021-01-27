use serde::de::{SeqAccess, DeserializeSeed};
use crate::de::error::TychoDeserializeError;
use crate::{Element, Value};
use crate::de::de::TychoDeserializer;
use crate::ident::ElementIdent;
use crate::encode::element::ElementEncoder;

pub struct TychoListDeserializer {
    list: Vec<Value>
}

impl TychoListDeserializer {
    pub(crate) fn new(list: Vec<Value>) -> Self {
        Self { list }
    }

    #[allow(dead_code)]
    pub(crate) fn from(e: Element) -> Result<Self, TychoDeserializeError> {
        if let Element::List(value) = e {
            Ok(Self::new(value))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::List,
                found: e.ident()
            })
        }
    }
}

impl<'a> SeqAccess<'a> for TychoListDeserializer {
    type Error = TychoDeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as DeserializeSeed<'a>>::Value>, Self::Error> where
        T: DeserializeSeed<'a> {
        if self.list.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(TychoDeserializer::new(Element::Value(self.list.remove(0))))?))
        }
    }
}

pub struct TychoArrayDeserializer {
    array: Vec<Element>
}

impl TychoArrayDeserializer {
    pub(crate) fn new(array: Vec<Element>) -> Self {
        Self { array }
    }

    #[allow(dead_code)]
    pub(crate) fn from(e: Element) -> Result<Self, TychoDeserializeError> {
        if let Element::Array(value) = e {
            Ok(Self::new(value))
        } else {
            Err(TychoDeserializeError::ExpectingElementType {
                expecting: ElementIdent::List,
                found: e.ident()
            })
        }
    }
}

impl<'a> SeqAccess<'a> for TychoArrayDeserializer {
    type Error = TychoDeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<<T as DeserializeSeed<'a>>::Value>, Self::Error> where
        T: DeserializeSeed<'a> {
        if self.array.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(seed.deserialize(TychoDeserializer::new(self.array.remove(0)))?))
        }
    }
}