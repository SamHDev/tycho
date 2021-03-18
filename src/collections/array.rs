use std::ops::{Deref, DerefMut};

use crate::Element;
use crate::into::value::ValueType;

/// Maps to `Vec<Value>` where items are homogeneous
#[derive(Debug)]
pub struct Array<T: ValueType>(pub Vec<T>);

impl<T: ValueType> From<Array<T>> for Element {
    fn from(a: Array<T>) -> Self {
        Element::Array(T::IDENT, a.0.into_iter().map(|x| x.into()).collect())
    }
}

impl<T: ValueType> Deref for Array<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: ValueType> DerefMut for Array<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: ValueType> Array<T> {
    pub fn new() -> Self { Self(Vec::new()) }
}
impl<T: ValueType> From<Vec<T>> for Array<T> {
    fn from(v: Vec<T>) -> Self {
        Self(v)
    }
}