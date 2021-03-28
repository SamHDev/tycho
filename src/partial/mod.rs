//! Partial unmarshall and traversal. (requires `partial` feature)
//!
//! ## Parital Reading
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

