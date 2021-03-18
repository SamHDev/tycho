//! Partial unmarshall and traversal. (requires `partial` feature)

pub use element::PartialElement;
pub use reader::{PartialPointer, PartialReader};
pub use types::{PartialArray, PartialList, PartialMap, PartialStruct};

//pub mod types;
pub mod container;
pub mod reader;
pub mod element;
pub mod types;
pub mod test;

