

use std::boxed::Box;

struct Pair {
    key: String,
    value: f32,
}

fn to_array(p: &[Pair]) -> Vec<String> {
    let map_size = p.len();
    let mut arr = Vec::with_capacity(map_size);

    for elem in p {
        arr.push(elem.key.clone());
    }

    arr
}

