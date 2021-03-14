use std::collections::HashMap;
use crate::Element;
use std::ops::Deref;
use std::convert::TryFrom;

pub struct Map<K: >(pub(crate) HashMap<String, Element>);

impl Deref for Struct {
    type Target = HashMap<String, Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Struct {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, field: &str, value: Element) {
        self.0.insert(field.to_string(), value);
    }

    pub fn remove(&mut self, field: &str) -> Option<Element> {
        self.0.remove(&field.to_string())
    }

    pub fn value<T: TryFrom<Element>>(&self, field: &str) -> Option<T> {
        T::try_from(self.0.get(field)?.clone()).ok()
    }

    pub fn get(&self, field: &str) -> Option<&Element> {
        self.0.get(field)
    }
}

impl From<Struct> for Element {
    fn from(s: Struct) -> Self {
        Self::Struct(s.0)
    }
}