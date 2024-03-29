use std::collections::HashMap;

use serde::ser::{SerializeStruct, Error};
use serde::Serialize;

use crate::{Element, Uuid, Number};
use crate::Value;
use crate::error::TychoError;
use crate::serde::ser::TychoSerializer;
use crate::serde::ser::seq::{SeqSerializer, SeqSerializerType};
use crate::types::ident::ValueIdent;
use crate::serde::ser::map::MapSerializer;

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

    #[cfg(feature="serde_types")]
    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {

        if key == "inner" {

            match self.name.as_str() {
                "__tycho__/array" => {
                    return if let Some(Element::Value(Value::Number(Number::Unsigned8(internal)))) = self.content.get("ident") {
                        if let Some(ident) = ValueIdent::from_internal_prefix(&internal) {
                            self.content.insert(
                                "inner".to_string(),
                                value.serialize(
                                    SeqSerializer::new(SeqSerializerType::Array(ident))
                                )?);
                            Ok(())
                        } else {
                            Err(Self::Error::custom("Invalid serde transfer type for Array."))
                        }
                    } else {
                        Err(Self::Error::custom("Invalid serde transfer type for Array."))
                    }
                },

                "__tycho__/map" => {
                    return if let Some(Element::Value(Value::Number(Number::Unsigned8(internal)))) = self.content.get("ident") {
                        if let Some(ident) = ValueIdent::from_internal_prefix(&internal) {
                            self.content.insert(
                                "inner".to_string(),
                                value.serialize(MapSerializer::typed(ident))?
                            );
                            Ok(())
                        } else {
                            Err(Self::Error::custom("Invalid serde transfer type for Array."))
                        }
                    } else {
                        Err(Self::Error::custom("Invalid serde transfer type for Array."))
                    }
                }

                _ => ()

            }

        }

        self.content.insert(key.to_string(), value.serialize(TychoSerializer)?);
        Ok(())
    }

    #[cfg(not(feature="serde_types"))]
    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.content.insert(key.to_string(), value.serialize(TychoSerializer)?);
        Ok(())
    }

    #[cfg(feature="serde_types")]
    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        match self.name.as_str() {
            "___tycho___/uuid" => if let Some(Element::Value(Value::Bytes(x))) = self.content.get("inner") {
                Ok(Element::Value(Value::UUID(Uuid::from_bytes(&x))))
            } else {
                Err(Self::Error::custom("Invalid serde transfer type for Uuid."))
            },
            "__tycho__/array" => if let Some(x) = self.content.remove("inner") {
                Ok(x)
            } else {
                Err(Self::Error::custom("Invalid serde transfer type for Array."))
            },
            "__tycho__/map" => if let Some(x) = self.content.remove("inner") {
                Ok(x)
            } else {
                Err(Self::Error::custom("Invalid serde transfer type for Map."))
            },
            _ => Ok(Element::Struct(self.content))
        }
    }

    #[cfg(not(feature="serde_types"))]
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Struct(self.content))
    }
}