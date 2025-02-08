
use std::collections::HashMap;

fn to_array(m: &HashMap<String, ()>) -> Vec<String> {
    let mut arr = Vec::with_capacity(m.len());
    for key in m.keys() {
        arr.push(key.clone());
    }
    arr.sort(); // To ensure consistent ordering
    arr
}

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("娀"), ());
    println!("{:?}", to_array(&map));
}
