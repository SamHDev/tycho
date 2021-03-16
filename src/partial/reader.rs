use std::io::{Read, Seek, SeekFrom, Cursor, Result as IoResult};
use crate::partial::element::PartialElement;
use crate::error::{TychoResult, TychoStatus, parse_io};


pub struct PartialReader<R: Read + Seek> {
    pub(crate) reader: R,
    pub(crate) pointer: u64
}

impl PartialReader<Cursor<Vec<u8>>> {
    pub fn from_vec(reader: Vec<u8>) -> Self {
        PartialReader {
            reader: Cursor::new(reader),
            pointer: 0
        }
    }
}

impl<R: Read + Seek> PartialReader<R> {
    pub fn from(reader: R) -> PartialReader<R> {
        PartialReader {
            reader,
            pointer: 0
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
    pub(crate) size: u64
}