use flate2::write::GzEncoder;
use flate2::Compression;
use crate::error::{TychoResult, TychoError};
use std::io::Write;

pub(crate) fn compress(bytes: Vec<u8>) -> TychoResult<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&bytes)?;
    match encoder.finish() {
        Ok(x) => Ok(x),
        Err(e) => Err(TychoError::Io(e))
    }
}