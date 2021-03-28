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