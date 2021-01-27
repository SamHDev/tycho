use crate::{Element, Value};

pub struct TychoArray(pub(crate) Vec<Element>);

impl TychoArray {
    pub fn new() -> Self { Self(Vec::new()) }
    pub fn push<V: Into<Element>>(&mut self, value: V) {
        self.0.push(value.into());
    }
    pub fn insert<V: Into<Element>>(&mut self, index: usize, value: V) {
        self.0.insert(index, value.into());
    }
    pub fn extend<V: Into<Element>>(&mut self, array: TychoArray) {
        self.0.extend_from_slice(&array.0);
    }
    pub fn remove(&mut self, index: usize) -> Option<Element> {
        self.0.get(index)?;
        Some(self.0.remove(index))
    }
    pub fn get(&self, index: usize) -> Option<&Element> {
        self.0.get(index)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Into<Element> for TychoArray {
    fn into(self) -> Element {
        Element::Array(self.0)
    }
}

impl<T: Into<Element>> Into<TychoArray> for Vec<T> {
    fn into(self) -> TychoArray {
        TychoArray(self.into_iter().map(|x| x.into()).collect())
    }
}



pub struct TychoList<T: Into<Value>>(pub(crate) Vec<T>);

impl<T: Into<Value>> TychoList<T> {
    pub fn new() -> Self { Self(Vec::new()) }
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }
    pub fn insert(&mut self, index: usize, value: T) {
        self.0.insert(index, value);
    }
    pub fn extend(&mut self, array: TychoList<T>) {
        for e in array.0 {
            self.0.push(e);
        };
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        self.0.get(index)?;
        Some(self.0.remove(index))
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub(crate) fn map(self) -> Vec<Value> {
        self.0.into_iter().map(|x| x.into()).collect()
    }
}

impl<T: Into<Value>> Into<Element> for TychoList<T> {
    fn into(self) -> Element {
        Element::List(self.map())
    }
}

impl<T: Into<Value>> Into<TychoArray> for TychoList<T> {
    fn into(self) -> TychoArray {
        let a = self.0.into_iter().map(|x| x.into().into()).collect();
        TychoArray(a)
    }
}

impl<T: Into<Value>> Into<TychoList<T>> for Vec<T> {
    fn into(self) -> TychoList<T> {
        TychoList(self)
    }
}