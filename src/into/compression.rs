//! Utility traits for element compression/decompression. (requires `compression` feature)

use crate::error::{TychoResult, TychoError};
use crate::Element;

/// Utility for element compression
pub trait CompressElement {
    /// Mark an element for compression by placing it within a compression element.
    fn compress(self) -> Self;
}

/// Utility for element decompression
pub trait DecompressElement: Sized {
    #[doc(hidden)]
    fn _impl_decompress(self) -> Result<Self, (Self, TychoError)>;

    /// Mark an element for decompression by extracting it from a compression element.
    fn decompress(self) -> TychoResult<Self> {
        match self._impl_decompress() {
            Ok(x) => Ok(x),
            Err(e) => Err(e.1)
        }
    }

    /// Mark an element for decompression by extracting it from a compression element.
    /// If the function fails, the element will be returned untouched.
    fn decompress_lossy(self) -> Self {
        match self._impl_decompress() {
            Ok(x) => x,
            Err(e) => e.0
        }
    }

    /// Mark an element for decompression by extracting it from a compression element.
    /// If the function fails, `None` is returned
    fn decompress_opt(self) -> Option<Self> {
        match self._impl_decompress() {
            Ok(x) => Some(x),
            Err(_e) => None
        }
    }
}

impl CompressElement for Element {
    fn compress(self) -> Self {
        Element::Compression(Box::new(self))
    }
}

impl DecompressElement for Element {
    fn _impl_decompress(self) -> Result<Self, (Self, TychoError)> {
        if let Element::Compression(e) = self {
            Ok(*e)
        } else {
            Err((self, TychoError::Other("Element cannot be decompressed".to_string())))
        }
    }
}