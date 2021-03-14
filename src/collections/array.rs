use std::collections::HashMap;
use crate::Element;
use std::ops::Deref;
use std::convert::TryFrom;
use crate::into::value::ValueType;
use std::hash::Hash;

pub struct

impl<T: ValueType> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ValueType> Array<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert(&mut self, key: K, value: Element) {
        self.0.insert(key, value);
    }

    pub fn remove(&mut self, field: &K) -> Option<Element> {
        self.0.remove(field)
    }

    pub fn value<T: TryFrom<Element>>(&self, field: &K) -> Option<T> {
        T::try_from(self.0.get(field)?.clone()).ok()
    }

    pub fn get(&self, field: &K) -> Option<&Element> {
        self.0.get(field)
    }
}

impl<K: ValueType + Eq + Hash> From<Map<K>> for Element {
    fn from(s: Map<K>) -> Self {
        Self::Map(K::IDENT, s.0.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}