
use std::cmp::Ordering;

struct Pair {
    key: String,
    value: f32,
}

struct OrderedMap(Vec<Pair>);

fn less(p: &OrderedMap, i: isize, j: isize) -> bool {
    p.0[i as usize].value < p.0[j as usize].value
}
