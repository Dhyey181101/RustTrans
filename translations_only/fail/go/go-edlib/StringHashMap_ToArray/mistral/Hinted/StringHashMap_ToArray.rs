
use std::collections::HashMap;

type StringHashMap = HashMap<String, Box<()>>;

fn to_array(hm: &StringHashMap) -> Vec<String> {
    let mut arr = Vec::new();
    let mut index = 0;
    for (key, _) in hm {
        arr.push(key.clone());
    }
    arr
}

fn insert_string_to_hashmap(hm: &mut StringHashMap, s: &str) {
    hm.insert(s.to_string(), Box::new(()));
}

fn main() {
    let mut m = StringHashMap::new();
    insert_string_to_hashmap(&mut m, "");
    insert_string_to_hashmap(&mut m, "\u{1F600}"); // emoji
    let arr = to_array(&m);
    println!("{:?}", arr);
}
