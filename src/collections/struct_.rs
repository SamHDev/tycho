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
    /// Create a new empty struct.
    pub fn new() -> Self { Self(HashMap::new()) }

    /// Insert an item into the struct.
    ///
    /// This function uses the `Into<Element>` trait.
    /// ```
    /// use tycho::collections::Struct;
    /// use tycho::Element;
    /// let mut s = Struct::new();
    /// s.insert("a", "foo");
    /// ```
    pub fn insert<V: Into<Element>>(&mut self, key: &str, value: V) -> Option<Element> {
        self.0.insert(key.to_string(), value.into())
    }

    /// Get an item from the struct.
    pub fn get(&self, key: &str) -> Option<&Element> {
        self.0.get(key)
    }

    /// Remove an item from the struct.
    pub fn remove(&mut self, key: &str) -> Option<Element> {
        self.0.remove(key)
    }

    /// Get a value from a struct with a given type
    ///
    /// ```
    /// use tycho::collections::Struct;
    /// use tycho::{Element, Value, Number};
    /// let mut s = Struct::new();
    ///
    /// // Insert
    /// s.insert("foo", 420i32);
    ///
    /// // Retrieve
    /// assert_eq!(s.get("foo"), Some(&Element::Value(Value::Number(Number::Signed32(420)))));
    /// //assert_eq!(s.value("foo"), Some(&420i32))
    /// ```
    ///
    #[doc(hidden)]
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

// serde

/*
#[cfg(feature="serde_support")]
use serde::{Serialize, Serializer, Deserialize, Deserializer};
#[cfg(feature="serde_support")]
use serde::ser::SerializeStruct;
#[cfg(feature="serde_support")]
use serde::de::{Visitor, MapAccess};
#[cfg(feature="serde_support")]
use std::fmt;


#[cfg(feature="serde_support")]
impl Serialize for Struct {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let mut s = serializer.serialize_struct("", self.0.len())?;
        for (k, v) in &self.0 {
            s.serialize_field(&k, &v)?;
        }
        s.end()
    }
}

#[cfg(feature="serde_support")]
impl<'de> Deserialize<'de> for Struct {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        unimplemented!()
    }
}

#[cfg(feature="serde_support")]
pub(crate) struct StructVisitor;


#[cfg(feature="serde_support")]
impl<'de> Visitor<'de> for Struct {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("structure")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, <A as MapAccess<'de>>::Error> where
        A: MapAccess<'de>,
    {
        let mut build = HashMap::new();
        while let Some((key, value)) = map.next()? {
            build.insert(key, value);
        }
        Ok(Struct(build))
    }
}

*/