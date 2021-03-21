use std::collections::HashMap;
use crate::{Value, Element};
use serde::ser::{SerializeMap, Error, SerializeStruct, SerializeSeq};
use crate::error::TychoError;
use serde::Serialize;
use crate::serde::ser::TychoSerializer;
use crate::into::ident::Ident;
use crate::ident::ValueIdent;
use crate::serde::ser::seq::{SeqSerializer, SeqSerializerType};

pub struct StructSerializer {
    content: HashMap<String, Element>,
    name: String
}

impl StructSerializer {
    pub fn new(name: &str) -> Self {
        Self {
            content: HashMap::new(),
            name: name.to_string()
        }
    }
}

impl SerializeStruct for StructSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.content.insert(key.to_string(), value.serialize(TychoSerializer)?);
        Ok(())
    }


    fn end(self) -> Result<Self::Ok, Self::Error> {
        match self.name.as_str() {
            _ => Ok(Element::Struct(self.content))
        }
    }
}