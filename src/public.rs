use std::io::{BufWriter, Cursor, Read, Write};

#[cfg(feature="async_tokio")]
pub use async_tokio_public::*;
#[cfg(feature="serde_support")]
pub use serde_public::*;

use crate::Element;
use crate::error::{TychoResult, TychoStatus};
use crate::read::element::read_element;
use crate::write::element::write_element;

/// Marshall an element to a byte buffer or writable object.
///
/// ### Example
/// ```
/// use tycho::{Element::Value, Value::Boolean, marshall};
/// use std::io::BufWriter;
///
/// // Create a boolean value.
/// let data = Value(Boolean(true));
///
/// // Create a buffer for result.
/// let mut buffer = BufWriter::new(Vec::new());
///
/// // Write boolean value to buffer.
/// marshall(&mut buffer, data).unwrap();
///
/// assert_eq!(buffer.buffer(), vec![1, 1, 1]);
/// ```
pub fn marshall<W: Write, E: Into<Element>>(writer: &mut W, element: E) -> TychoStatus {
    write_element(writer, &element.into())
}

/// Marshall an element into a vec of bytes.
///
/// ### Example
/// ```
/// use tycho::{Element::Value, Value::Char, marshall_vec};
///
/// // Create a char value.
/// let data = Value(Char('@'));
///
/// // marshall the value into a vec.
/// let bytes = marshall_vec(data).unwrap();
///
/// assert_eq!(bytes, vec![1, 3, 64]);
/// ```
pub fn marshall_vec<E: Into<Element>>(element: E) -> TychoResult<Vec<u8>> {
    let mut buffer = BufWriter::new(Vec::new());
    marshall(&mut buffer, element)?;
    Ok(buffer.into_inner().unwrap()) // todo: issue may occur here not sure. will catch later.
}

/// Unmarshall an element from a readable object.
///
/// ### Example
/// ```
/// use std::io::{BufReader, Cursor};
/// use tycho::unmarshall;
/// use tycho::Value::Number;
/// use tycho::Number::Unsigned8;
/// use tycho::Element::Value;
///
/// // Create a cursor with example bytes
/// let mut bytes = Cursor::new(vec![1, 4, 1, 10]);
///
/// // Unmarshall readable object.
/// let data = unmarshall(&mut bytes).unwrap();
///
/// assert_eq!(data, Value(Number(Unsigned8(10))));
/// ```
pub fn unmarshall<R: Read>(reader: &mut R) -> TychoResult<Element> {
    read_element(reader)
}


/// Unmarshall an element from a vec of bytes.
///
/// ### Example
/// ```
/// use tycho::unmarshall_vec;
/// # use tycho::Number::Unsigned16;
/// # use tycho::Element::Value;
/// # use tycho::Value::Number;
///
/// // Example bytes
/// let bytes = vec![1, 4, 2, 1, 164];
///
/// // Unmarshall bytes into vec.
/// let data = unmarshall_vec(bytes).unwrap();
///
/// assert_eq!(data, Value(Number(Unsigned16(420))))
/// ```
pub fn unmarshall_vec(data: Vec<u8>) -> TychoResult<Element>  {
    let mut buffer = Cursor::new(data);
    unmarshall(&mut buffer)
}



#[cfg(feature="async_tokio")]
mod async_tokio_public {
    use tokio::io::AsyncRead;

    use crate::Element;
    use crate::error::TychoResult;
    use crate::read::async_::element::read_element_async;

    /// Unmarshall an element from a async readable object.
    ///
    /// ### Example
    /// ```
    /// use std::io::{BufReader, Cursor};
    /// use tycho::unmarshall_async;
    /// # use tycho::Number::Unsigned16;
    /// # use tycho::Element::Value;
    /// # use tycho::Value::Number;
    /// # let _ = tokio_test::block_on( async {
    ///
    /// // Create a cursor with example bytes
    /// let mut bytes = Cursor::new(vec![1, 4, 2, 1, 164]);
    ///
    /// // Unmarshall cursor
    /// let data = unmarshall_async(&mut bytes).await.unwrap();
    ///
    /// assert_eq!(data, Value(Number(Unsigned16(420))));
    /// # });
    /// ```
    ///
    /// ### File Example
    /// ```no_run
    /// use std::io::{BufReader, Cursor, Read};
    /// use tycho::unmarshall_async;
    /// # use tycho::Number::Unsigned16;
    /// # use tycho::Element::Value;
    /// # use tycho::Value::Number;
    /// # let _ = tokio_test::block_on( async {
    ///
    /// // Create a cursor with example bytes
    /// let mut bytes = tokio::fs::File::open("number.tycho").await.unwrap();
    ///
    /// // Unmarshall cursor
    /// let data = unmarshall_async(&mut bytes).await.unwrap();
    ///
    /// assert_eq!(data, Value(Number(Unsigned16(420))));
    /// # });
    /// ```
    ///
    pub async fn unmarshall_async<R: AsyncRead + Unpin + Send>(reader: &mut R) -> TychoResult<Element> {
        read_element_async(reader).await
    }

}


#[cfg(feature="serde_support")]
mod serde_public {
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    use crate::{Element, marshall_vec, unmarshall_vec};
    use crate::error::TychoResult;
    use crate::serde::de::TychoDeserializer;
    use crate::serde::ser::TychoSerializer;

    /// Serialize a serde serializable object into an Element. (requires `serde_support`)
    ///
    /// ```
    /// use serde::Serialize;
    /// use tycho::{to_element, Element};
    /// use tycho::collections::Struct;
    ///
    /// // Create a serializable serde structure.
    /// #[derive(Serialize)]
    /// pub struct Example {
    ///     foo: String
    /// }
    ///
    /// // Instantiate serializable object.
    /// let data = Example { foo: "Hi".to_string() };
    ///
    /// // Convert serializable serde structure to Elements.
    /// let element = to_element(data).unwrap();
    ///
    /// // Create structure manually (for assert_eq)
    /// let mut map = Struct::new();
    /// map.insert("foo", "Hi");
    ///
    /// assert_eq!(element, map.into())
    /// ```
    pub fn to_element<S: Serialize>(o: S) -> TychoResult<Element> {
        o.serialize(TychoSerializer)
    }

    /// Serialize a serde serializable object into tycho bytes.  (requires `serde_support`)
    ///
    /// ```
    /// use serde::Serialize;
    /// use tycho::{to_element, Element, to_bytes};
    /// use tycho::collections::Struct;
    ///
    /// // Create a serializable serde structure.
    /// #[derive(Serialize)]
    /// pub struct Example {
    ///     foo: String
    /// }
    ///
    /// // Instantiate serializable object.
    /// let data = Example { foo: "Hi".to_string() };
    ///
    /// // Convert serializable serde structure to bytes.
    /// let bytes = to_bytes(data).unwrap();
    ///
    /// assert_eq!(bytes, vec![5, 9, 102, 111, 111, 0, 1, 2, 2, 72, 105])
    /// ```
    pub fn to_bytes<S: Serialize>(o: S) -> TychoResult<Vec<u8>> {
        marshall_vec(to_element(o)?)
    }

    /// Deserialize an element into a serde deserializable object. (requires `serde_support`)
    ///
    /// ```
    /// use serde::Deserialize;
    /// use tycho::{to_element, Element, to_bytes, from_element};
    /// use tycho::collections::Struct;
    ///
    /// // Create a serializable serde structure.
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// pub struct Example {
    ///     foo: String
    /// }
    ///
    /// // Instantiate object manually.
    /// let mut map = Struct::new();
    /// map.insert("foo", "Hi");
    ///
    /// let example: Example = from_element(map).unwrap();
    ///
    /// assert_eq!(example, Example { foo: "Hi".to_string() })
    /// ```
    pub fn from_element<D: DeserializeOwned, E: Into<Element>>(e: E) -> TychoResult<D> {
        D::deserialize(TychoDeserializer::new(e.into()))
    }

    /// Deserialize tycho bytes into a serde deserializable object. (requires `serde_support`)
    pub fn from_bytes<D: DeserializeOwned>(b: &[u8]) -> TychoResult<D> {
        from_element(unmarshall_vec(b.to_vec())?)
    }
}

