use std::collections::HashMap;
use crate::Element;
use std::ops::{DerefMut, Deref};

/// Maps to `HashMap<String, Element>`
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