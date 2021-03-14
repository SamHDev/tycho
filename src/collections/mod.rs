//! Wrappers around `HashMap` and `Vec` mapping to a respective type within tycho.
//!
//!
//! These functions come in handy when creating objects/elements manually or want a specific
//! serialisation target when using serde.

use std::collections::HashMap;
use crate::Element;
use crate::into::value::ValueType;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

/// Maps to `HashMap<String, Element>`
pub struct Struct(pub HashMap<String, Element>);

/// Maps to `HashMap<Value, Element>` where value is homogeneous
pub struct Map<K: ValueType + Hash + Eq>(pub HashMap<K, Element>);

/// Maps to `Vec<Element>` where items are heterogeneous
pub struct List(pub Vec<Element>);

/// Maps to `Vec<Value>` where items are homogeneous
pub struct Array<T: ValueType>(pub Vec<T>);


impl From<Struct> for Element {
    fn from(s: Struct) -> Self {
        Element::Struct(s.0)
    }
}
impl<K: ValueType + Hash + Eq> From<Map<K>> for Element {
    fn from(m: Map<K>) -> Self {
        Element::Map(K::IDENT, m.0.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }
}
impl From<List> for Element {
    fn from(l: List) -> Self {
        Element::List(l.0)
    }
}
impl<T: ValueType> From<Array<T>> for Element {
    fn from(a: Array<T>) -> Self {
        Element::Array(T::IDENT, a.0.into_iter().map(|x| x.into()).collect())
    }
}

impl Deref for Struct {
    type Target = HashMap<String, Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<K: ValueType + Hash + Eq> Deref for Map<K> {
    type Target = HashMap<K, Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deref for List {
    type Target = Vec<Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: ValueType> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Struct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<K: ValueType + Hash + Eq> DerefMut for Map<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<K: ValueType> DerefMut for Array<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

