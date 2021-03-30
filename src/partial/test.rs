#[test]
fn test() {
    use crate::collections::Struct;
    use crate::marshall_vec;
    use crate::partial::PartialElement;
    use crate::partial::PartialReader;

    let mut data = Struct::new();

    data.insert("foo", 10u8);
    data.insert("bar", 20u16);
    data.insert("baz", "Hello World");

    //println!("{:?}", data);

    let bytes = marshall_vec(data).unwrap();

    println!("{:?}", bytes);

    let mut reader = PartialReader::from_vec(bytes);
    let root = reader.element().unwrap();

    if let PartialElement::Struct(mut s) = root {
        //println!("{:?}", s.pointer);
        for (_key, _value) in s.iter(&mut reader) {
            //println!("{}: {:?}", key, value);
        }
        s.top();
        //println!("{:?}", s.collect(&mut reader));
    }
}
