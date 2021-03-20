use std::collections::HashMap;
use crate::{Value, Element};
use serde::ser::{SerializeMap, Error};
use crate::error::TychoError;
use serde::Serialize;
use crate::serde::ser::TychoSerializer;
use crate::into::ident::Ident;
use crate::ident::ValueIdent;

pub struct MapSerializer {
    content: HashMap<Value, Element>,
    map_type: ValueIdent,
    key: Option<Value>
}

impl MapSerializer {
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            map_type: ValueIdent::Null,
            key: None
        }
    }
}

impl SerializeMap for MapSerializer {
    type Ok = Element;
    type Error = TychoError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
        T: Serialize {
        let data = key.serialize(TychoSerializer)?;

        if let Element::Value(value) = data {
            if self.map_type == ValueIdent::Null {
              self.map_type = value.ident();
            } else if value.ident() != self.map_type {
                return Err(TychoError::MismatchedType { found: value.ident(), expected: self.map_type.clone() })
            }
            self.key = Some(value);
        } else {
            return Err(TychoError::InvalidKeyType { found: data.ident() })
        }

        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {

        let value = value.serialize(TychoSerializer)?;
        let key = std::mem::replace(&mut self.key, None);
        if let Some(k) = key {
            self.content.insert(k, value)
        } else {
            Err(TychoError::custom("Invalid key state."))
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(Element::Map(self.map_type, self.content))
    }
}