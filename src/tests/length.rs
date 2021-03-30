use std::io::{BufWriter, Cursor};

use crate::read::length::read_length;
use crate::write::length::write_length;

fn encode_var_length(length: usize) -> Vec<u8> {
    let mut buffer = BufWriter::new(Vec::new());
    write_length(&mut buffer, length).unwrap();
    return buffer.buffer().to_vec();
}

fn decode_var_length(bytes: Vec<u8>) -> usize {
    let mut buffer = Cursor::new(bytes);
    return read_length(&mut buffer).unwrap();
}

macro_rules! sample_test {
    ($name: expr, $value: expr, $bytes: expr) => {
        paste::item! {
            #[test]
            fn [< varlength_ $name _encode >]() {
                assert_eq!(encode_var_length($value), $bytes)
            }
            #[test]
            fn [< varlength_ $name _decode >]() {
                assert_eq!(decode_var_length($bytes), $value)
            }
        }
    };
}

sample_test!(0, 0, vec![0]);
sample_test!(1, 1, vec![1]);
sample_test!(2, 2, vec![2]);
sample_test!(3, 127, vec![127]);
sample_test!(4, 128, vec![128, 1]);
sample_test!(5, 255, vec![255, 1]);
sample_test!(6, 2097151, vec![255, 255, 127]);
sample_test!(7, 2147483647, vec![255, 255, 255, 255, 7]);
