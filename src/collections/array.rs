use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use crate::{Element, Value};
use crate::into::value::ValueType;

/// Maps to `Vec<Value>` where items are homogeneous
#[derive(Debug)]
pub struct Array<T: ValueType>(pub Vec<T>);

impl<T: ValueType> From<Array<T>> for Element {
    fn from(a: Array<T>) -> Self {
        Element::Array(T::IDENT, a.0.into_iter().map(|x| x.into()).collect())
    }
}

impl<T: ValueType> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: ValueType> DerefMut for Array<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ValueType> Array<T> {
    /// Create a new empty array.
    pub fn new() -> Self { Self(Vec::new()) }
}
impl<T: ValueType> From<Vec<T>> for Array<T> {
    /// Create a new array from an existing Vec.
    fn from(v: Vec<T>) -> Self {
        Self(v)
    }
}

impl<K: ValueType + TryFrom<Value>> TryFrom<Element> for Array<K> {
    type Error = ();

    fn try_from(value: Element) -> Result<Self, Self::Error> {
        if let Element::Array(ident, set) = value {
            if K::IDENT == ident {
                Ok(Array(set.into_iter()
                    .filter_map(|x| K::try_from(x).ok() )
                    .collect()
                ))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

#[cfg(feature="serde_support")]
use serde::{Serialize, Serializer};
#[cfg(feature="serde_support")]
use serde::ser::SerializeStruct;
use serde::ser::SerializeSeq;

#[cfg(all(feature="serde_support", feature="serde_types"))]
impl<K: ValueType + TryFrom<Value> + Serialize> Serialize for Array<K> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut stu = serializer.serialize_struct("___tycho___/array", 2)?;
        stu.serialize_field("ident", &K::IDENT.to_internal_prefix())?;
        stu.serialize_field("inner", &self.0)?;
        stu.end()
    }
}

#[cfg(all(feature="serde_support", not(feature="serde_types")))]
impl<K: ValueType + TryFrom<Value> + Serialize> Serialize for Array<K> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut seq = serializer.serialize_seq(Some(self.0.len()))?;
        for x in self.0 {
            seq.serialize_element(x)?;
        }
        seq.end()
    }
}