
use std::collections::HashMap;

pub fn to_array(map: &HashMap<String, f32>) -> Vec<String> {
    let mut arr = Vec::with_capacity(map.len());
    for (key, _) in map {
        arr.push(key.clone());
    }
    arr
}

pub fn len(map: &HashMap<String, f32>) -> usize {
    map.len()
}

#[derive(Debug)]
pub struct Pair {
    pub key: String,
    pub value: f32,
}

pub type OrderedMap = Vec<Pair>;
