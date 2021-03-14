use std::io::{Read, Cursor, Write};
use crate::collections::Struct;
use crate::marshall_vec;

pub struct PartialHandle<T> {
    buffer: Vec<u8>,
    buffer_position: usize,
    cursor_position: usize,
    reader: T
}

impl<T> PartialHandle<T> {
    pub fn new(handle: T) {

    }
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



}
