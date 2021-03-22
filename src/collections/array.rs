use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

use crate::{Element, Value};
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

impl<K: ValueType + TryFrom<Value>> TryFrom<Element> for Array<K> {
    type Error = ();

    fn try_from(value: Element) -> Result<Self, Self::Error> {
        if let Element::Array(ident, set) = value {
            if K::IDENT == ident {
                Ok(Array(set.into_iter()
                    .filter_map(|x| K::try_from(x).ok() )
                    .collect()
                ))
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}