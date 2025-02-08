
use std::collections::HashMap;

type StringHashMap = HashMap<String, ()>;

fn to_array(m: &StringHashMap) -> Vec<String> {
    let mut arr: Vec<String> = Vec::with_capacity(m.len());
    for key in m.keys() {
        arr.push(key.clone());
    }
    arr
}
