use std::ops::{Deref, DerefMut};

use crate::Element;

/// Maps to `Vec<Element>` where items are heterogeneous
#[derive(Debug)]
pub struct List(pub Vec<Element>);

impl From<List> for Element {
    fn from(l: List) -> Self {
        Element::List(l.0)
    }
}

impl Deref for List {
    type Target = Vec<Element>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for List {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl List {
    pub fn new() -> Self { Self(Vec::new()) }
}
impl From<Vec<Element>> for List {
    fn from(v: Vec<Element>) -> Self {
        Self(v)
    }
}