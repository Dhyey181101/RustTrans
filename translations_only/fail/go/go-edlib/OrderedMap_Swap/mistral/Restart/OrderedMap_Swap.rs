

use std::vec::Vec;

struct Pair {
key: String,
value: f32,
}

type OrderedMap = Vec<Pair>;

fn swap(v: &mut OrderedMap, i: usize, j: usize) {
v.swap(i, j);
}

