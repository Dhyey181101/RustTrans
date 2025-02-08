
use std::collections::HashMap;

type StringHashMap = HashMap<String, ()>;

fn to_array(m: &StringHashMap) -> Vec<String> {
    let mut arr = Vec::with_capacity(m.len());
    for key in m.keys() {
        arr.push(key.clone());
    }
    arr
}

fn main() {
    let mut map = StringHashMap::new();
    map.insert("key1".to_string(), ());
    map.insert("key2".to_string(), ());
    let arr = to_array(&map);
    println!("{:?}", arr);
}
