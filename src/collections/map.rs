use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::{Element, Value};
use crate::into::value::ValueType;

/// Maps to `HashMap<Value, Element>` where value is homogeneous
#[derive(Debug)]
pub struct Map<K: ValueType + Hash + Eq>(pub HashMap<K, Element>);

impl<K: ValueType + Hash + Eq> From<Map<K>> for Element {
    fn from(m: Map<K>) -> Self {
        Element::Map(K::IDENT, m.0.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}

impl<K: ValueType + Hash + Eq> Deref for Map<K> {
    type Target = HashMap<K, Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K: ValueType + Hash + Eq> DerefMut for Map<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ValueType + Hash + Eq> Map<T> {
    /// Create a new empty map.
    pub fn new() -> Self { Self(HashMap::new()) }
}
impl<T: ValueType + Hash + Eq> From<HashMap<T, Element>> for Map<T> {
    fn from(v: HashMap<T, Element>) -> Self {
        Self(v)
    }
}

impl<K: ValueType + Hash + Eq + TryFrom<Value>> TryFrom<Element> for Map<K> {
    type Error = ();

    fn try_from(value: Element) -> Result<Self, Self::Error> {
        if let Element::Map(ident, map) = value {
            if K::IDENT == ident {
                Ok(Map(map.into_iter()
                    .filter_map(|(k, v)|
                        Some((K::try_from(k).ok()?, v)))
                    .collect()))
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
#[cfg(feature="serde_support")]
use serde::ser::SerializeSeq;
use serde::ser::SerializeMap;

#[cfg(all(feature="serde_support", feature="serde_types"))]
impl<K: ValueType + Hash + Eq + TryFrom<Value> + Serialize> Serialize for Map<K> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut stu = serializer.serialize_struct("___tycho___/map", 2)?;
        stu.serialize_field("ident", &K::IDENT.to_internal_prefix())?;
        stu.serialize_field("inner", &self.0)?;
        stu.end()
    }
}

#[cfg(all(feature="serde_support", not(feature="serde_types")))]
impl<K: ValueType + Hash + Eq + TryFrom<Value> + Serialize> Serialize for Map<K> {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut seq = serializer.serialize_map(Some(self.0.len()))?;
        for (k, v) in self.0 {
            seq.serialize_entry(k ,&v)?;
        }
        seq.end()
    }
}

