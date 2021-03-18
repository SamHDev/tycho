use std::io::{Cursor, Read, Result as IoResult, Seek, SeekFrom};

#[cfg(feature="partial_state")]
use rand;

use crate::error::{parse_io, TychoResult, TychoStatus};
use crate::partial::element::PartialElement;

pub struct PartialReader<R: Read + Seek> {
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

impl<R: Read + Seek> PartialReader<R> {
    pub fn from(reader: R) -> PartialReader<R> {
        PartialReader {
            reader,
            pointer: 0,

            #[cfg(feature="partial_state")]
            ident: rand::random()
        }
    }

    pub fn jump(&mut self, to: &u64) -> TychoStatus {
        parse_io(self.reader.seek(SeekFrom::Current((*to as i64) - (self.pointer as i64))))?;
        self.pointer = *to;
        Ok(())
    }

    pub fn element(&mut self) -> TychoResult<PartialElement> {
        PartialElement::read(self)
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
pub struct PartialPointer {
    pub(crate) pos: u64,
    pub(crate) size: u64,

    #[cfg(feature="partial_state")]
    pub(crate) ident: u16,
}