use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Example {
    foo: String,
    bar: u8,
    baz: bool
}

#[test]
fn example() {
    let example = Example {
        foo: "Hello World".to_string(),
        bar: 10,
        baz: true
    };


    let elements = crate::to_element(example).unwrap();

    let bytes = crate::marshall_vec(elements.clone()).unwrap();

    println!("{:?}", elements);
    println!("{:?}", bytes);
    println!("{}", bytes.into_iter().map(|x| format!("{:02x?}", x)).collect::<Vec<String>>().join(" "))


}

