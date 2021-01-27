use serde::ser::SerializeMap;
use crate::{Element, Value};
use crate::ser::error::TychoSerializerError;
use serde::Serialize;
use crate::ser::ser::TychoSerializer;
use crate::values::ValueCanHash;
use crate::ident::ValueIdent;
use std::collections::HashMap;
use crate::encode::value::ValueEncoder;
use crate::encode::element::ElementEncoder;

pub struct TychoMapSerializer {
    key_type: Option<ValueIdent>,
    last_key: Option<Value>,
    data: HashMap<Value, Element>
}

impl TychoMapSerializer {
    pub(crate) fn new() -> Self {
        Self { key_type: None, last_key: None, data: HashMap::new() }
    }

    pub(crate) fn insert_key<T: ?Sized>(&mut self, key: &T) -> Result<(), TychoSerializerError> where
        T: Serialize {
        let ser = key.serialize(TychoSerializer)?;

        if let Element::Value(v) = ser.clone() {
            match &self.key_type {
                None => {
                    self.key_type = Some(v.ident());
                    if !v.can_hash() {
                        return Err(TychoSerializerError::InvalidValueType {
                            found: ser.ident().clone(),
                            reason: String::from("Map key serialize returned a unhashable value.")
                        })
                    }
                },
                Some(ident) => {
                    if ident != &v.ident() {
                        return Err(TychoSerializerError::ValueTypeMismatch {
                            expected: ident.clone(),
                            found: v.ident()
                        })
                    }
                }
            }
            self.last_key = Some(v);
            Ok(())
        } else {
            return Err(TychoSerializerError::InvalidValueType {
                found: ser.ident(),
                reason: String::from("Map key serialize returned a non-value element.")
            })
        }
    }

    pub(crate) fn insert_value<T: ?Sized>(&mut self, value: &T) -> Result<(), TychoSerializerError> where
        T: Serialize {
        match &self.last_key {
            None => return Err(TychoSerializerError::NoKeyGiven),
            Some(key) => {
                self.data.insert(key.clone(), value.serialize(TychoSerializer)?);
                Ok(())
            }
        }
    }

    pub(crate) fn finish(self) -> Result<Element, TychoSerializerError> {
        Ok(Element::Map(self.data))
    }
}

impl SerializeMap for TychoMapSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.insert_key(key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.insert_value(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}