
use std::collections::HashMap;

fn to_array(m: HashMap<String, ()>) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    let mut index: usize = 0;
    for key in m.keys() {
        arr.push(key.to_string());
        index += 1;
    }

    return arr;
}

type StringHashMap = HashMap<String, ()>;
