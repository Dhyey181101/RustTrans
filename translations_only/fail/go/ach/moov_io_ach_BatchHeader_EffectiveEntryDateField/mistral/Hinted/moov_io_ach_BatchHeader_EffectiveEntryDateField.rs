

use std::fmt;
use std::boxed::Box;

fn main() {
    let mut map = std::collections::HashMap::new();
    map.insert(1, Box::new(10));
    map.insert(2, Box::new(20));
    map.insert(3, Box::new(30));

    for (k, v) in &map {
        println!("Key: {}, Value: {}", k, v);
    }

    let v1 = **map.get(&1).unwrap();
    println!("Value of key 1: {}", v1);

    let v2 = map.remove(&2).unwrap();
    println!("Value of key 2: {}", v2);
}

struct MyStruct {
    value: i32,
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct: {}", self.value)
    }
}

fn create_my_struct() -> Box<MyStruct> {
    Box::new(MyStruct { value: 100 })
}

