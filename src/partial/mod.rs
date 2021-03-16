//! Partial unmarshall and traversal. (requires `partial` feature)

//pub mod types;
pub mod container;
pub mod reader;
pub mod element;
pub mod types;
pub mod test;

pub use reader::{PartialReader, PartialPointer};
pub use element::PartialElement;
pub use types::{PartialStruct, PartialArray, PartialList, PartialMap};