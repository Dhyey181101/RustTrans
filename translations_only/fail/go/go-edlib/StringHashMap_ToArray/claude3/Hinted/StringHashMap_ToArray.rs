
use std::collections::HashSet;

fn to_array(m: &HashSet<String>) -> Vec<String> {
    let mut arr = Vec::with_capacity(m.len());
    for key in m.iter() {
        arr.push(key.clone());
    }
    arr
}

type StringHashMap = HashSet<String>;

fn main() {
    let mut set = StringHashMap::new();
    set.insert(String::from("hello"));
    set.insert(String::from("world"));
    let arr = to_array(&set);
    println!("{:?}", arr);
}
