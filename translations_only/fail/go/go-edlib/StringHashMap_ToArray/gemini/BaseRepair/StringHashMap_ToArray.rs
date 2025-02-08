
use std::collections::HashMap;

pub fn to_array(m: &HashMap<String, ()>) -> Vec<String> {
    let mut arr = Vec::with_capacity(m.len());
    for key in m.keys() {
        arr.push(key.clone());
    }
    arr
}

pub type StringHashMap = HashMap<String, ()>;
