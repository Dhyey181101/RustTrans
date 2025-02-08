
use std::boxed::Box;

#[derive(Debug)]
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Pair>;

fn to_array(p: &OrderedMap) -> Vec<String> {
    let map_size = p.len();
    let mut arr = Vec::with_capacity(map_size);
    for elem in p.iter() {
        arr.push(elem.key.clone());
    }
    arr
}

fn len(p: &OrderedMap) -> usize {
    p.len()
}
