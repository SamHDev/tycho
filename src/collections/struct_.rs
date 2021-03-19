use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use crate::Element;

/// Maps to `HashMap<String, Element>`
#[derive(Debug)]
pub struct Struct(pub HashMap<String, Element>);

impl From<Struct> for Element {
    fn from(s: Struct) -> Self {
        Element::Struct(s.0)
    }
}

impl Deref for Struct {
    type Target = HashMap<String, Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Struct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Struct {
    pub fn new() -> Self { Self(HashMap::new()) }

    pub fn insert<V: Into<Element>>(&mut self, key: &str, value: V) -> Option<Element> {
        self.0.insert(key.to_string(), value.into())
    }

    pub fn get(&self, key: &str) -> Option<&Element> {
        self.0.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<Element> {
        self.0.remove(key)
    }

    pub fn value<'x, V: From<&'x Element>>(&'x self, key: &str) -> Option<V> {
        match V::try_from(self.0.get(key)?) {
            Ok(x) => Some(x),
            Err(_) => None
        }
    }
}
impl From<HashMap<String, Element>> for Struct {
    fn from(v: HashMap<String, Element>) -> Self {
        Self(v)
    }
}

impl TryFrom<Element> for Struct {
    type Error = ();

    fn try_from(value: Element) -> Result<Self, Self::Error> {
        if let Element::Struct(map) = value {
            Ok(Struct(map))
        } else {
            Err(())
        }
    }
}