

use std::vec::Vec;

type Pair = (String, f32);
type OrderedMap = Vec<Pair>;

fn to_array(p: &OrderedMap) -> Vec<String> {
    let map_size = p.len();
    let mut arr = vec![String::new(); map_size];

    for (i, (ref key, _)) in p.iter().enumerate() {
        arr[i] = key.clone();
    }

    arr
}

fn len(p: &OrderedMap) -> usize {
    p.len()
}

fn main() {}

