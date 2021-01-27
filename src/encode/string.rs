use crate::encode::util::join_bytes;
use crate::encode::length::encode_variable_length;

pub fn encode_string(string: &str) -> Vec<u8> {
    let data = string.to_string().into_bytes();
    join_bytes(encode_variable_length(data.len() as u32), &data)
}

pub fn encode_term_string(string: &str) -> Vec<u8> {
    let mut data = string.to_string().into_bytes();
    data.push(0);
    data
}