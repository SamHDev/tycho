use serde::de::Visitor;
use serde::Deserializer;
use serde::forward_to_deserialize_any;

use crate::error::TychoError;

pub struct TychoIdentDeserializer(String);

impl TychoIdentDeserializer {
    pub fn new(x: &str) -> Self {
        Self(x.to_string())
    }
}

/*macro_rules! to_any {
    ($ident: expr) => {
        paste::item! {
            fn [< deserialize_ $ident >]<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
               self.deserialize_any(visitor)
            }

        }
    };
}*/

impl<'de> Deserializer<'de> for TychoIdentDeserializer {
    type Error = TychoError;

    fn deserialize_any<V>(self, visitor: V) -> Result<<V as Visitor<'de>>::Value, Self::Error> where V: Visitor<'de> {
        visitor.visit_string(self.0)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}
