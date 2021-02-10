use crate::decode::reader::Reader;
use crate::decode::error::DecodeError;

pub(crate) fn read_var_length(reader: &mut Reader) -> Result<u32, DecodeError> {
    let a = reader.read_one()?;
    let size = a >> 6;
    let mut length = ((a as u32) & 0x3F);
    for (_, x) in reader.read_many(size as usize)?.into_iter().enumerate() {
        length <<= 8u32;
        length += x as u32;
    }
    Ok(length)
}