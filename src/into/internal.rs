use crate::types::ident::{ValueIdent, NumberIdent};

#[allow(dead_code)]
impl ValueIdent {
    pub(crate) fn to_internal_prefix(&self) -> u8 {
        match &self {
            ValueIdent::Null => 0x00,
            ValueIdent::Boolean => 0x01,
            ValueIdent::String => 0x02,
            ValueIdent::Char => 0x03,
            ValueIdent::Number(num) => match num {
                NumberIdent::Bit => 0xF0,
                NumberIdent::Unsigned8 => 0xF1,
                NumberIdent::Signed8 => 0xF2,
                NumberIdent::Unsigned16 => 0xF3,
                NumberIdent::Signed16 => 0xF4,
                NumberIdent::Unsigned32 => 0xF5,
                NumberIdent::Signed32 => 0xF6,
                NumberIdent::Unsigned64 => 0xF7,
                NumberIdent::Signed64 => 0xF8,
                NumberIdent::Unsigned128 => 0xF9,
                NumberIdent::Signed128 => 0xFA,
                NumberIdent::Float32 => 0xFB,
                NumberIdent::Float64 => 0xFC,
            }
            ValueIdent::Bytes => 0x05,
            ValueIdent::UUID => 0x06,
        }
    }

    pub(crate) fn from_internal_prefix(x: &u8) -> Option<Self> {
        match x {
            0x00 => Some(ValueIdent::Null),
            0x01 => Some(ValueIdent::Boolean),
            0x02 => Some(ValueIdent::String),
            0x03 => Some(ValueIdent::Char),
            0x05 => Some(ValueIdent::Bytes),
            0x06 => Some(ValueIdent::UUID),
            0xF0 => Some(ValueIdent::Number(NumberIdent::Bit)),
            0xF1 => Some(ValueIdent::Number(NumberIdent::Unsigned8)),
            0xF2 => Some(ValueIdent::Number(NumberIdent::Signed8)),
            0xF3 => Some(ValueIdent::Number(NumberIdent::Unsigned16)),
            0xF4 => Some(ValueIdent::Number(NumberIdent::Signed16)),
            0xF5 => Some(ValueIdent::Number(NumberIdent::Unsigned32)),
            0xF6 => Some(ValueIdent::Number(NumberIdent::Signed32)),
            0xF7 => Some(ValueIdent::Number(NumberIdent::Unsigned64)),
            0xF8 => Some(ValueIdent::Number(NumberIdent::Signed64)),
            0xF9 => Some(ValueIdent::Number(NumberIdent::Unsigned128)),
            0xFA => Some(ValueIdent::Number(NumberIdent::Signed128)),
            0xFB => Some(ValueIdent::Number(NumberIdent::Float32)),
            0xFC => Some(ValueIdent::Number(NumberIdent::Float64)),
            _ => None
        }
    }
}