use std::io::{Read, Cursor, Write};
use crate::collections::Struct;
use crate::{marshall_vec, Value};
use byteorder::ReadBytesExt;

pub enum PartialReadError {
    IoError(std::io::Error),
    PointerFlushed { head: usize, position: usize }
}

pub struct PartialSource<R: Read> {
    buffer: Vec<u8>,
    head: usize,
    read: R
}

impl PartialSource<Cursor<Vec<u8>>> {
    pub fn with_bytes(buffer: Vec<u8>) -> Self {
        let cursor = Cursor::new(buffer);
        Self::with_reader(cursor)
    }
}

impl<R: Read> PartialSource<R> {
    pub fn with_reader(read: R) -> Self { Self { buffer: vec![], head: 0, read } }


    pub fn read_at(&mut self, pos: usize) -> Result<PartialReadError, E> {
        match
    }

    pub fn flush(&mut self) {
        self.head += self.buffer.len();
        self.buffer.clear();
    }

    pub fn head(&mut self) -> PartialPointer<R> {

    }
}


pub struct PartialPointer<R: Read> {
    position: usize,
    size: usize
}

pub enum PartialElement<R: Read> {

}


#[test]
fn test() {
    let mut data = Struct::new();

    data.insert("foo", 10u8);
    data.insert("bar", 20u16);
    data.insert("baz", "Hello World");

    println!("{:?}", data);

    let bytes = marshall_vec(data).unwrap();

    println!("{:?}", bytes);

    let mut reader = PartialSource::with_bytes(bytes);

    let name = reader.head();
    // println!("{:?}", name)
}
