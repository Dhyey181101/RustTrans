
use std::collections::HashMap;
use std::fmt;
use once_cell::sync::Lazy;

static HASHMAP: Lazy<HashMap<i32, i32>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(1, 2);
    map.insert(3, 4);
    map
});

struct MyStruct;

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyStruct")
    }
}

fn main() {
    let _ = HASHMAP;
    let my_struct = MyStruct;
    println!("{}", my_struct);
}
