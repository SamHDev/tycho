use crate::decode::reader::Reader;
use crate::decode::error::DecodeError;

pub(crate) fn read_var_length(reader: &mut Reader) -> Result<u32, DecodeError> {
    let a = reader.read_one()?;
    let size = a >> 6;
    let mut length = ((a as u32) & 0x3F) << (size * 8);
    for (i, x) in reader.read_many(size as usize)?.into_iter().enumerate() {
        length += (x as u32) << ((size - (i as u8)) * 8);
    }
    Ok(length)
}