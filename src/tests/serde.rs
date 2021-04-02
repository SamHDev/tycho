use crate::{Uuid, marshall_vec, to_element, from_element, Element};
use serde::{Serialize, Deserialize};

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

