//! Wrappers around `HashMap` and `Vec` mapping to a respective type within tycho.
//!
//!
//! These functions come in handy when creating objects/elements manually or want a specific
//! serialisation target when using serde.

pub(crate) mod array;
pub(crate) mod list;
pub(crate) mod map;
pub(crate) mod struct_;

pub use array::Array;
pub use list::List;
pub use map::Map;
pub use struct_::Struct;