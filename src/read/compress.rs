use crate::error::{TychoResult, TychoError};
use flate2::write::GzDecoder;
use std::io::Write;

pub(crate) fn decompress(bytes: Vec<u8>) -> TychoResult<Vec<u8>> {
    let mut decoder = GzDecoder::new(Vec::new());
    decoder.write_all(&bytes)?;
    match decoder.finish() {
        Ok(x) => Ok(x),
        Err(e) => Err(TychoError::Io(e))
    }
}