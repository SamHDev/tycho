use crate::into::value::ValueType;
use std::collections::HashMap;
use std::hash::Hash;
use crate::Element;
use std::ops::{Deref, DerefMut};

/// Maps to `HashMap<Value, Element>` where value is homogeneous
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