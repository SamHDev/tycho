use crate::{Element, Value};
use std::collections::HashMap;
use std::hash::Hash;

pub struct TychoStruct(pub(crate) HashMap<String, Element>);

impl TychoStruct {
    pub fn new() -> Self { Self(HashMap::new()) }
    pub fn insert<V: Into<Element>>(&mut self, key: &str, value: V) {
        self.0.insert(key.to_string(), value.into());
    }
    pub fn remove(&mut self, key: &str) -> Option<Element> {
        self.0.remove(&key.to_string())
    }
    pub fn get(&mut self, key: &str) -> Option<&Element> {
        self.0.get(&key.to_string())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Into<Element> for TychoStruct {
    fn into(self) -> Element {
        Element::Struct(self.0)
    }
}

pub struct TychoMap<K: Into<Value> + Hash + Eq, V: Into<Element>>(pub(crate) HashMap<K, V>);

impl<K: Into<Value> + Hash + Eq, V: Into<Element>> TychoMap<K, V> {
    pub fn new() -> Self { Self(HashMap::new()) }
    pub fn insert(&mut self, key: K, value: V) {
        self.0.insert(key, value);
    }
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.0.remove(key)
    }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn map(self) -> HashMap<Value, Element> {
        self.0.into_iter().map(|(k, v)| (k.into(), v.into())).collect()
    }
}

impl<K: Into<Value> + Hash + Eq, V: Into<Element>> Into<Element> for TychoMap<K, V> {
    fn into(self) -> Element {
        Element::Map(self.map())
    }
}