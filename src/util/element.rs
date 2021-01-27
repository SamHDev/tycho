use crate::{Value, Element};
use crate::util::{TychoArray, TychoList, TychoStruct, TychoMap};
use std::hash::Hash;

pub trait ElementUtil {
    fn unit() -> Self;
    fn value<T: Into<Value>>(value: T) -> Self;

    fn option<T: Into<Element>>(inner: Option<T>) -> Self;
    fn none() -> Self;
    fn some<T: Into<Element>>(inner: T) -> Self;

    fn array<A: Into<TychoArray>>(array: A) -> Self;
    fn list<A: Into<TychoList<T>>, T: Into<Value>>(array: A) -> Self;

    fn structure(structure: TychoStruct) -> Self;
    fn map<K: Into<Value> + Hash + Eq, V: Into<Element>>(map: TychoMap<K, V>) -> Self;

    fn variant<V: Into<Element>>(name: &str, value: V) -> Self;
}

impl ElementUtil for Element {
    fn unit() -> Self {
        Self::Unit
    }

    fn value<T: Into<Value>>(value: T) -> Self {
        Self::Value(value.into())
    }

    fn option<T: Into<Element>>(inner: Option<T>) -> Self {
        match inner {
            Some(value) => Self::Option(Some(Box::new(value.into()))),
            None => Self::Option(None)
        }
    }

    fn none() -> Self {
       Self::Option(None)
    }

    fn some<T: Into<Element>>(inner: T) -> Self {
        Self::Option(Some(Box::new(inner.into())))
    }

    fn array<A: Into<TychoArray>>(array: A) -> Self {
        array.into().into()
    }

    fn list<A: Into<TychoList<T>>, T: Into<Value>>(array: A) -> Self {
        array.into().into()
    }

    fn structure(structure: TychoStruct) -> Self {
        structure.into()
    }

    fn map<K: Into<Value> + Hash + Eq, V: Into<Element>>(map: TychoMap<K, V>) -> Self {
        map.into()
    }

    fn variant<V: Into<Element>>(name: &str, value: V) -> Self {
        Self::Variant(name.to_string(), Box::new(value.into()))
    }
}