pub mod ident;
pub(crate) mod element;
pub(crate) mod encode;
pub mod value;
pub(crate) mod display;
pub(crate) mod decode;

#[cfg(feature="compression")]
pub mod compression;

mod internal;