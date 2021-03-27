//! Partial unmarshall and traversal. (requires `partial` feature)

#[cfg(feature = "async_tokio")]
pub use async_::reader::PartialAsyncReader;

pub use element::PartialElement;
pub use reader::{PartialPointer, PartialReader};
pub use types::{PartialArray, PartialList, PartialMap, PartialStruct};

//pub mod types;
pub(crate) mod container;
pub(crate) mod reader;
pub(crate) mod element;
pub(crate) mod types;
pub(crate) mod test;

#[cfg(feature = "async_tokio")]
pub(crate) mod async_;

