
use std::collections::HashMap;

fn to_array(m: HashMap<String, ()>) -> Vec<String> {
    let mut index = 0;
    let mut arr = Vec::new();
    for key in m.keys() {
        arr.push(key.to_string());
        index += 1;
    }

    return arr;
}

type StringHashMap = HashMap<String, ()>;
