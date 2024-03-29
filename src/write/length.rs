use std::io::Write;

use crate::error::TychoStatus;
use crate::write::func::write_byte;

pub(crate) fn write_length<W: Write>(writer: &mut W, mut length: usize) -> TychoStatus {
    loop {

        let write = (length & 0x7F) as u8;
        //println!("{:?} {:?}", length, write);
        length >>= 7;

        if length == 0 {
            return write_byte(writer, &write);
        } else {
            write_byte(writer, &(write | 0x80))?;
        }
    }
}
