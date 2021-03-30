use std::io::{Cursor, Read, Result as IoResult, Seek, SeekFrom};

#[cfg(feature="partial_state")]
use rand;

use crate::error::{parse_io, TychoResult, TychoStatus};
use crate::partial::element::{PartialElement, read_partial_element};

/// A reader with an inner pointer and state management for reading tycho partially.
///
/// PartialReader can take two types of reader:
/// - sync (`std::io::Read` + `std::io::Seek`)
/// - async (`tokio::io:AsyncRead` + `tokio::io::AsyncSeek`)
///
/// When using async, functions are suffixed with a `_async`.
/// For example `element()` would become `element_async().await`
///
/// ### Creating
/// You can create a partial reader by giving any sized type.
/// ```
/// use std::io::Cursor;
/// use tycho::partial::PartialReader;
/// let cursor = Cursor::new(vec![]);
///
/// let mut reader = PartialReader::from(reader);
/// ```
///
/// You can also give a vec of bytes:
///
/// ```
/// use tycho::partial::PartialReader;
/// let mut reader = PartialReader::from_vec(vec![]);
/// ```
///
/// ### Reading
///
/// You can get the inital/root element of the reader by calling `.element()`
/// ```
/// use tycho::partial::PartialReader;
/// let mut reader = PartialReader::from_vec(vec![]);
///
/// let element = reader.element().unwrap();
/// ```
pub struct PartialReader<R> {
    pub(crate) reader: R,
    pub(crate) pointer: u64,

    #[cfg(feature="partial_state")]
    pub(crate) ident: u16,
}

impl PartialReader<Cursor<Vec<u8>>> {
    pub fn from_vec(reader: Vec<u8>) -> Self {
        PartialReader {
            reader: Cursor::new(reader),
            pointer: 0,

            #[cfg(feature="partial_state")]
            ident: rand::random()
        }
    }
}

// sync implementation
impl<R: Read + Seek> PartialReader<R> {

    /// Jump to a pointer location.
    pub fn jump(&mut self, to: &u64) -> TychoStatus {
        parse_io(self.reader.seek(SeekFrom::Current((*to as i64) - (self.pointer as i64))))?;
        self.pointer = *to;
        Ok(())
    }

    /// Get the next element of the reader.
    pub fn element(&mut self) -> TychoResult<PartialElement> {
        read_partial_element(self)

    }

}

/*
impl<R: Read + Seek + Write> PartialReader<R> {
    pub fn snip(&mut self, pointer: PartialPointer) -> TychoResult<Vec<u8>> {
        #[cfg(feature="partial_state")]
        if self.pointer.ident != reader.ident {
            return Err(TychoError::OutdatedPointer)
        }

        self.jump(&pointer.pos)?;

        self.reader.rem2
    }
}*/

impl<R> PartialReader<R> {
    pub fn from(reader: R) -> PartialReader<R> {
        PartialReader {
            reader,
            pointer: 0,

            #[cfg(feature = "partial_state")]
            ident: rand::random()
        }
    }

    pub(crate) fn pointer(&self, pos: u64, size: u64) -> PartialPointer {
        PartialPointer {
            pos,
            size,
            #[cfg(feature="partial_state")]
            ident: self.ident.clone()
        }
    }

    pub(crate) fn empty_pointer(&self) -> PartialPointer {
        PartialPointer {
            pos: 0,
            size: 0,
            #[cfg(feature="partial_state")]
            ident: self.ident.clone()
        }
    }
}

impl<R: Read + Seek> Read for PartialReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        let size = self.reader.read(buf)?;
        self.pointer += size.clone() as u64;
        Ok(size)
    }
}


#[derive(Debug, Clone)]
/// A pointer, referring to a block of data in a partial reader.
pub struct PartialPointer {
    pub(crate) pos: u64,
    pub(crate) size: u64,

    #[cfg(feature="partial_state")]
    pub(crate) ident: u16,
}