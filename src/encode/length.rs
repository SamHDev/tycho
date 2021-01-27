// 2bit length + 30 bit data
pub fn encode_variable_length(size: u32) -> Vec<u8> {
    let size = size & 0x3FFFFFFF;
    let length: usize = if size < 0x3F { 0 }
    else if size < 0x3FFF { 1 }
    else if size < 0x03FFFFF { 2 }
    else { 3 };

    let mut bytes = size.to_be_bytes()[(3-length)..4].to_vec();

    bytes[0] = bytes[0] | ((length as u8) << 6);
    bytes
}

#[cfg(test)]
mod length_tests {
    use crate::encode::length::encode_variable_length;

    #[test]
    fn length_test_0() {
        assert_eq!(encode_variable_length(1), vec![1]);
    }
    #[test]
    fn length_test_1() {
        assert_eq!(encode_variable_length(64), vec![64, 64]);
    }
    #[test]
    fn length_test_2() {
        assert_eq!(encode_variable_length(255), vec![64, 255]);
    }
    #[test]
    fn length_test_3() {
        assert_eq!(encode_variable_length(1024), vec![68, 0]);
    }
    #[test]
    fn length_test_4() {
        assert_eq!(encode_variable_length(42069), vec![128, 164, 85]);
    }
    #[test]
    fn length_test_5() {
        assert_eq!(encode_variable_length(42069420), vec![194, 129, 237, 172]);
    }
}