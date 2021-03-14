use crate::{marshall_vec};
use std::str::FromStr;

#[test]
fn encode_bool_false() {
    assert_eq!(marshall_vec(false).unwrap(), vec![1, 1, 0]);
}

#[test]
fn encode_bool_true() {
    assert_eq!(marshall_vec(true).unwrap(), vec![1, 1, 1]);
}

#[test]
fn encode_char_ascii() {
    assert_eq!(marshall_vec('A').unwrap(), vec![1, 3, 65]);
}

#[test]
fn encode_char_emoji() {
    assert_eq!(marshall_vec('🚀').unwrap(), vec![1, 3, 240, 159, 154, 128]);
}

#[test]
fn encode_string_ascii() {
    assert_eq!(
        marshall_vec("Hello World!").unwrap(),
        vec![1, 2, 12, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100, 33]);
}

#[test]
fn encode_string_emoji() {
    assert_eq!(
        marshall_vec("🎮 !Gamers unite! 🎮").unwrap(),
        vec![1, 2, 24, 240, 159, 142, 174, 32, 33, 71, 97, 109, 101, 114, 115, 32, 117, 110, 105,
             116, 101, 33, 32, 240, 159, 142, 174]);
}

#[test]
fn encode_uuid() {
    let uuid = uuid::Uuid::from_str("13c5ded9-50af-4cf7-81e1-5e1f57a58b4c").unwrap();
    assert_eq!(
        marshall_vec(uuid).unwrap(),
        vec![1, 6, 19, 197, 222, 217, 80, 175, 76, 247, 129, 225, 94, 31, 87, 165, 139, 76]);

}

#[test]
fn encode_bytes() {
    assert_eq!(
        marshall_vec(vec![10_u8, 20_u8, 30_u8, 40_u8, 50_u8]).unwrap(),
        vec![1, 5, 5, 10, 20, 30, 40, 50]);

}