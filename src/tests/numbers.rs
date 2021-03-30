use crate::{Element, marshall_vec, Number, unmarshall_vec, Value};

macro_rules! number_test {
    ($name: expr, $ident: ident, $test: expr, $bytes: expr) => {
        paste::item! {
            #[test]
            fn [< number_ $name _encode >]() {
                 let value = Element::Value(Value::Number(Number::$ident($test)));

                let data = marshall_vec(value).unwrap();
                assert_eq!(data, $bytes);
            }
            #[test]
            fn [< number_ $name _decode >]() {
                let data = unmarshall_vec($bytes).unwrap();
                if let Element::Value(Value::Number(Number::$ident(v))) = data {
                    assert_eq!(v, $test)
                } else {
                    panic!("Bad result {:?}", data)
                }
            }
        }
    };
}


number_test!(u8_min, Unsigned8, 0, vec![1, 4, 1, 0]);
number_test!(u8_max, Unsigned8, 255, vec![1, 4, 1, 255]);
number_test!(u8_norm, Unsigned8, 69, vec![1, 4, 1, 69]);

/*number_test!(i8_min, Signed8, -128, vec![1, 4, 17, 0]);
number_test!(i8_max, Signed8, 127, vec![1, 4, 17, 255]);
number_test!(i8_zero, Signed8, 0, vec![1, 4, 17, 127]);
number_test!(i8_norm, Signed8, 69, vec![1, 4, 17, 69]);*/