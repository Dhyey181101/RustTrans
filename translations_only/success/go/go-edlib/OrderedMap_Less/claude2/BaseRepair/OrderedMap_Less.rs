
use std::cmp::Ordering;

struct Pair {
    key: String,
    value: f32,
}

struct OrderedMap(Vec<Pair>);

fn less(map: &OrderedMap, i: isize, j: isize) -> bool {
    map.0[i as usize].value < map.0[j as usize].value
}
