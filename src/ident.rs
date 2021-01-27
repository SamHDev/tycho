pub trait Ident<T> where Self: Sized {
    fn ident(&self) -> T;
    fn parse(value: &T) -> Option<Self>;
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum ValueIdent {
    Boolean = 0,

    Unsigned8 = 1,
    Unsigned16 = 2,
    Unsigned32 = 3,
    Unsigned64 = 4,
    Unsigned128 = 5,

    Signed8 = 6,
    Signed16 = 7,
    Signed32 = 8,
    Signed64 = 9,
    Signed128 = 10,

    Float32 = 11,
    Float64 = 12,

    String = 13,
    Char = 14,
    Bytes = 15,

    NIL = 0xFD
}

impl Ident<u8> for ValueIdent {
    fn ident(&self) -> u8 {
        self.clone() as u8
    }

    fn parse(value: &u8) -> Option<Self> {
        Some(match *value {
            x if x == Self::Boolean as u8 => Self::Boolean,

            x if x == Self::Unsigned8 as u8 => Self::Unsigned8,
            x if x == Self::Unsigned16 as u8 => Self::Unsigned16,
            x if x == Self::Unsigned32 as u8 => Self::Unsigned32,
            x if x == Self::Unsigned64 as u8 => Self::Unsigned64,
            x if x == Self::Unsigned128 as u8 => Self::Unsigned128,

            x if x == Self::Signed8 as u8 => Self::Signed8,
            x if x == Self::Signed16 as u8 => Self::Signed16,
            x if x == Self::Signed32 as u8 => Self::Signed32,
            x if x == Self::Signed64 as u8 => Self::Signed64,
            x if x == Self::Signed128 as u8 => Self::Signed128,

            x if x == Self::Float32 as u8 => Self::Float32,
            x if x == Self::Float64 as u8 => Self::Float64,

            x if x == Self::String as u8 => Self::String,
            x if x == Self::Char as u8 => Self::Char,
            x if x == Self::Bytes as u8 => Self::Bytes,
            _ => return None
        })
    }
}


#[repr(u8)]
#[derive(Debug, Clone, PartialEq)]
pub enum ElementIdent {
    Unit = 0,
    Value = 1,
    Option = 2,
    Array = 3,
    Struct = 4,
    Variant = 5,
    Map = 6,
    List = 7
}

impl Ident<u8> for ElementIdent {
    fn ident(&self) -> u8 {
        self.clone() as u8
    }

    fn parse(value: &u8) -> Option<Self> {
        Some(match *value {
            x if x == Self::Unit as u8 => Self::Unit,
            x if x == Self::Value as u8 => Self::Value,
            x if x == Self::Option as u8 => Self::Option,
            x if x == Self::Array as u8 => Self::Array,
            x if x == Self::Struct as u8 => Self::Struct,
            x if x == Self::Variant as u8 => Self::Variant,
            x if x == Self::Map as u8 => Self::Map,
            x if x == Self::List as u8 => Self::List,
            _ => return None
        })
    }
}