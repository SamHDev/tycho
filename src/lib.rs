//! # Tycho Binary Format
//! The tycho binary format is a minimal, self-describing and traversable data format designed
//! around rust and the serde data model.
//!
//! ### Features
//! - `partial` - Partial Reading/Traversal
//! - `serde` - Serde serialisation and deserialisation
//! - `serde_optimise` - Serde structure optimisation (default)
//! - `serde_types` - Serde structure type serialisation (default)
//! - `async_tokio` - Async reading support with tokio
//! - `compression` - Compression (gzip)

#![allow(unused_imports)]

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

#[cfg(feature="serde")]
mod serde;

pub mod collections;
pub mod error;

#[cfg(test)]
mod tests;

#[cfg(feature="uuid")]
pub(crate) mod uuid;
pub use crate::uuid::Uuid;
