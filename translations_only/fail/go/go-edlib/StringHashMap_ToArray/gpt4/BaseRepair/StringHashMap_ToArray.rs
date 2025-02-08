
use std::collections::HashMap;

struct StringHashMap(HashMap<String, ()>);

fn to_array(m: &StringHashMap) -> Vec<String> {
    let mut arr: Vec<String> = Vec::with_capacity(m.0.len());
    for key in m.0.keys() {
        arr.push(key.clone());
    }
    arr
}
