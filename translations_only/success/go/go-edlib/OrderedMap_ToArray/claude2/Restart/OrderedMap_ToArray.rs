
use std::boxed::Box;

struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Box<[Pair]>;

fn to_array(map: OrderedMap) -> Vec<String> {
    let mut arr = Vec::with_capacity(map.len());
    for pair in map.iter() {
        arr.push(pair.key.clone());
    }
    arr
}

fn len(map: &OrderedMap) -> isize {
    map.len() as isize
}
