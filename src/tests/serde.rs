use crate::{Uuid, marshall_vec, to_element, from_element, Element};
use serde::{Serialize, Deserialize};
use crate::collections::Array;

#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
    foo: Uuid
}

#[test]
fn test_serde_uuid() {
    let a = Example { foo: Uuid::v4() };
    println!("{:?}", a);

    let e = to_element(a).unwrap();
    println!("{:?}", e);

    println!("{:?}", from_element::<Example, Element>(e));
}


#[test]
fn test_serde_array() {
    let mut a = Array::new();
    a.push(10);
    a.push(20);
    println!("{:?}", a);

    let e = to_element(a).unwrap();
    println!("{:?}", e);

    //println!("{:?}", from_element::<Array<i32>, Element>(e));
}


