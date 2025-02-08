
use std::collections::HashMap;
use std::vec::Vec;

pub fn to_array(p: &HashMap<String, f32>) -> Vec<String> {
    let map_size = p.len();
    let mut arr = Vec::with_capacity(map_size);
    for (key, _) in p {
        arr.push(key.clone());
    }

    arr
}

pub fn len(p: &HashMap<String, f32>) -> usize {
    p.len()
}

#[derive(Clone)]
pub struct Pair {
    pub key: String,
    pub value: f32,
}

pub type OrderedMap = Vec<Pair>;
