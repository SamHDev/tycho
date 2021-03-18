use std::io::{BufWriter, Cursor, Read, Write};

#[cfg(feature="async_tokio")]
pub use async_tokio_public::*;

use crate::Element;
use crate::error::{TychoResult, TychoStatus};
use crate::read::element::read_element;
use crate::write::element::write_element;

/// Marshall an element to a byte buffer or writable object.
pub fn marshall<W: Write, E: Into<Element>>(writer: &mut W, element: E) -> TychoStatus {
    write_element(writer, &element.into())
}

/// Marshall an element into a vec of bytes.
pub fn marshall_vec<E: Into<Element>>(element: E) -> TychoResult<Vec<u8>> {
    let mut buffer = BufWriter::new(Vec::new());
    marshall(&mut buffer, element)?;
    Ok(buffer.into_inner().unwrap()) // todo: issue may occur here not sure. will catch later.
}

/// Unmarshall an element from a readable object.
pub fn unmarshall<R: Read>(reader: &mut R) -> TychoResult<Element> {
    read_element(reader)
}


/// Unmarshall an element from a vec of bytes.
pub fn unmarshall_vec(data: Vec<u8>) -> TychoResult<Element>  {
    let mut buffer = Cursor::new(data);
    unmarshall(&mut buffer)
}



#[cfg(feature="async_tokio")]
mod async_tokio_public {
    use std::io::Cursor;

    use tokio::io::AsyncRead;

    use crate::Element;
    use crate::error::TychoResult;
    use crate::read::async_tokio::element::read_element_async;

    /// Unmarshall an element from a async readable object.
    pub async fn unmarshall_async<R: AsyncRead + Unpin + Send>(reader: &mut R) -> TychoResult<Element> {
        read_element_async(reader).await
    }


    /// Unmarshall an element from a vec of bytes.
    pub async fn unmarshall_vec_async(data: Vec<u8>) -> TychoResult<Element>  {
        let mut buffer = Cursor::new(data);
        unmarshall_async(&mut buffer).await
    }
}

