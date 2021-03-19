use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use crate::{Element, Value};
use serde::Serialize;
use crate::error::TychoError;
use crate::ident::ValueIdent;
use crate::serde::ser::TychoSerializer;
use crate::into::ident::Ident;

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
        if self.array_opt && self.array_type == SeqSerializerType::Default {
            if let Element::Value(x) = &value {
                if self.array_type = ValueIdent::Null {
                    self.array_type = x.ident();
                } else if &x.ident() != self.array_type {
                    self.array_opt = false;
                }
            } else {
                self.array_opt = false;
            }
        }

        self.elements.push(value);
        Ok(())
    }

    pub fn end(self) -> Result<Element, TychoError> {
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
                Ok(Element::Array(ident,  self.elements
                    .into_iter()
                    .filter_map(|x|
                        if let Element::Value(v) = x { Some(v) } else { None })
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
