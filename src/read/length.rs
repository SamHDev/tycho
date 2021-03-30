use std::io::Read;

use crate::error::TychoResult;
use crate::read::func::read_byte;

pub(crate) fn read_length<R: Read>(reader: &mut R) -> TychoResult<usize> {
    let mut number: u64 = 0;
    let mut count = 0;

    loop {
        let byte = read_byte(reader)?;

        number |= ((byte & 0x7F) as u64) << (7 * count);

        if byte & 0x80 == 0 {
            return Ok(number as usize);
        }

        count += 1;
    }
}