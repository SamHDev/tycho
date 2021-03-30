//! Partial unmarshall and traversal. (requires `partial` feature)
//!
//!
//! ## Appliactions
//! Parital reading **is** for:
//! - Reading large tycho encoded files.
//!
//! Parital reading **is not** for:
//! - Reading from streams/sockets.
//!
//! Tycho Parital reading is designed for reading from large files where proccessing is limited
//! such as in a database or data store, where the contents do not need to be serialized.
//!
//! ## Quick Example
//! ```
//! use tycho::partial::{PartialReader, PartialElement};
//!
//! // Create a partial reader from a vec with example bytes.
//! let mut reader = PartialReader::from_vec(vec![
//!     /* ... */
//! # 5, 35, 102, 111, 111, 0, 1, 4, 1, 10, 98, 97, 114, 0, 1, 4, 2, 0, 20, 98, 97, 122, 0, 1, 2, 11, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100
//! ]);
//!
//! // Get the root element from
//! let root = reader.element().unwrap();
//!
//! // check if the root element is a structure
//! if let PartialElement::Struct(mut s) = root {
//!
//!     // iterate over fields with reader.
//!     for (key, value) in s.iter(&mut reader) {
//!         println!("{:?}: {:?}", key, value);
//!     }
//! }
//!
//! ```
//!
//! ## Getting Started
//!
//! ### Reader
//! Parital reading requires a `ParitalReader` to be created.
//! The partial reader has an internal pointer and handles state management.
//!
//! A `ParitalReader` can wrap any type that implements `Read` and `Seek`
//! and thier respective async counterparts.
//!
//! `ParitalReader` then implements its own `Read` trait that handles the pointer when read.
//!
//! ```
//! // From a Vec<u8>
//! use tycho::partial::PartialReader;
//!
//! let mut reader = ParitalReader::from_vec(vec![ /* ... */ ]);
//! ```
//!
//! ```
//! // From a cursor
//! use std::io::Cursor;
//! use tycho::partial::PartialReader;
//!
//! let cursor = Cursor::new(vec![ /* ... */ ]);
//! let mut reader = PartialReader::from(cursor);
//! ```
//! ```no_run
//! // From a file
//! use std::io::{Cursor, BufReader};
//! use tycho::partial::PartialReader;
//! use std::fs::File;
//!
//! let file = File::open("path/to/file").unwrap();
//! let buf = BufReader::new(file);
//! let mut reader = PartialReader::from(buf);
//! ```
//!
//! ### Pointers
//! Pointers map to a set of bytes within a reader.
//! They contain a `position` and `size`,
//! with position representing the start loaction and size the data length.
//!
//! Pointers are given out within a ParitalElement and wrapped within a container type.
//! While the user may not interact with a pointer directly it may help with understanding how
//! tycho paritally parses bytes.
//!
//! ### Element
//! ParitalElements contain a proccessed value or a unproccess container with its respective pointer.
//!
//! Within this libary there are four types of partial element:
//! - Proccessed (Unit, Value)
//! - Pointing (Option, Variant)
//! - Container (Struct, List, Map, Array)
//! - Compression (Compression)
//!
//! Proccessed values can be accesed by pattern matching the `ParitalElement` enum,
//! which will return a normaal value.
//!
//! Pointing elements return a box with another partial element which can be pattern matched
//! and handled accordingly.
//!
//! Container elements allow you to iterate over thier children one at a time.
//! See below for more infomation
//!
//! Compression elements allow you to get the bytes or another partial element upon request.
//!
//! ### Containers
//! All container types (Struct, List, Map, Array) share a `PartialContainer` which takes a generic.
//!
//! The PartialContainer type has another `head` pointer to keep track of the current item.
//!
//! **Further documentation soon*


#[cfg(feature = "async_tokio")]
pub use async_::reader::PartialAsyncReader;

pub use element::PartialElement;
pub use reader::{PartialPointer, PartialReader};
//pub use types::{PartialArray, PartialList, PartialMap, PartialStruct};

//pub mod types;
pub mod container;
pub(crate) mod reader;
pub(crate) mod element;
pub mod types;

//pub(crate) mod test;

#[cfg(feature = "async_tokio")]
pub(crate) mod async_;

