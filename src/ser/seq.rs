use serde::Serialize;
use crate::ser::ser::TychoSerializer;
use crate::ser::error::TychoSerializerError;
use crate::encode::element::ElementEncoder;
use crate::Element;
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};

pub struct TychoSeqSerializer {
    data: Vec<Element>,
    type_ident: Option<u8>,
    is_same: bool,
    is_values: bool
}

impl TychoSeqSerializer {
    pub(crate) fn new() -> Self {
        Self {
            data: Vec::new(),
            type_ident: None,
            is_same: true,
            is_values: true
        }
    }

    pub(crate) fn insert<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), TychoSerializerError> {
        let v = value.serialize(TychoSerializer)?;
        self.optimise_value(&v);
        self.data.push(v);
        Ok(())
    }

    pub(crate) fn optimise_value(&mut self, v: &Element) {
        if self.is_same {
            match self.type_ident {
                None => { self.type_ident = Some(v.prefix()) },
                Some(ident) => if ident != v.prefix() {
                    self.is_same = false;
                }
            }
            if self.is_values {
                if let Element::Value(_) = v {} else { self.is_values = false; }
            }
        }
    }

    pub(crate) fn finish(self) -> Result<Element, TychoSerializerError> {
        Ok(if self.data.len() == 0 || !(self.is_values && self.is_same) {
            Element::Array(self.data)
        } else {
            Element::List(self.data.into_iter()
                .filter_map(|x| if let Element::Value(v) = x { Some(v) }
                else { None }).collect())
        })
    }

}

impl SerializeSeq for TychoSeqSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.insert(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl SerializeTuple for TychoSeqSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.insert(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

impl SerializeTupleStruct for TychoSeqSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.insert(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}

pub struct TychoVariantSeqSerializer {
    inner: TychoSeqSerializer,
    name: String
}

impl TychoVariantSeqSerializer {
    pub(crate) fn new(name: &str) -> Self {
        Self {
            inner: TychoSeqSerializer::new(),
            name: name.to_string()
        }
    }

    pub(crate) fn insert<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), TychoSerializerError> {
        self.inner.insert(value)
    }

    pub(crate) fn finish(self) -> Result<Element, TychoSerializerError> {
        Ok(Element::Variant(self.name, Box::new(self.inner.finish()?)))
    }
}

impl SerializeTupleVariant for TychoVariantSeqSerializer {
    type Ok = Element;
    type Error = TychoSerializerError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where
        T: Serialize {
        self.insert(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.finish()
    }
}
