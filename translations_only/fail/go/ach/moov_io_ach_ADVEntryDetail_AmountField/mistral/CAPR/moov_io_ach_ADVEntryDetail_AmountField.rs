
use std::collections::HashMap;
use std::fmt;
use std::str;

const ZEROS: &[u8] = b"0000000000000000000";

struct MyStruct {
    data: HashMap<i32, String>,
}

impl MyStruct {
    fn new() -> MyStruct {
        MyStruct {
            data: HashMap::new(),
        }
    }

    fn insert(&mut self, key: i32, value: String) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &i32) -> Option<&String> {
        self.data.get(key)
    }
}

impl fmt::Debug for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct(data: {{")?;
        for (key, value) in &self.data {
            write!(f, "{}: {:?},", key, value)?;
        }
        write!(f, "}}")
    }
}

fn main() {
    let mut my_struct = MyStruct::new();
    my_struct.insert(1, "hello".to_string());
    my_struct.insert(2, "world".to_string());
    println!("{:?}", my_struct);
    println!("{}", str::from_utf8(ZEROS).unwrap());
}
