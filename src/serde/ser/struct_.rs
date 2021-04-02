use std::collections::HashMap;

use serde::ser::{SerializeStruct, Error};
use serde::Serialize;

use crate::{Element, Uuid};
use crate::Value;
use crate::error::TychoError;
use crate::serde::ser::TychoSerializer;

#[allow(dead_code)]
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
        println!("{} {:?}", self.name, self.content);
        match self.name.as_str() {
            "___tycho___/uuid" => if let Some(Element::Value(Value::Bytes(x))) = self.content.get("inner") {
                Ok(Element::Value(Value::UUID(Uuid::from_bytes(&x))))
            } else {
                Err(Self::Error::custom("Invalid serde transfer type for Uuid."))
            }
            _ => Ok(Element::Struct(self.content))
        }

    }
}