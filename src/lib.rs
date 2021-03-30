//! # Tycho Binary Format
//! The tycho binary format is a minimal, self-describing and traversable data format designed
//! around rust and the serde data model.
//!
//! ### Features
//! - `partial` - Partial Reading/Traversal
//! - `serde_support` - Serde serialisation and deserialisation
//! - `serde_optimise` - Serde structure optimisation (default)
//! - `async_tokio` - Async reading support with tokio
//! - `compression` - Compression (gzip)
//!

pub use public::*;
pub use types::ident;
pub use types::types::*;

pub(crate) mod types;

pub(crate) mod write;
pub(crate) mod read;
pub(crate) mod into;
pub(crate) mod public;

#[cfg(feature="partial")]
pub mod partial;

#[cfg(feature="serde_support")]
mod serde;

pub mod collections;
pub mod error;

#[cfg(test)]
mod tests;
