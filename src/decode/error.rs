use std::string::FromUtf8Error;

#[derive(Debug, Clone)]
pub enum DecodeError {
    UnexpectedEnd { expected: usize, found: usize },
    StringDecodeError { pos: usize, error: FromUtf8Error },
    BadCharLength { pos: usize },
    LogicalDecodeError { pos: usize },
    InvalidElementIdent { pos: usize, value: u8 },
    InvalidValueIdent { pos: usize, value: u8 },
    InvalidElementOperand { pos: usize, value: u8}
}