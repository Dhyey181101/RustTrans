

use std::cmp::Ordering;

struct Pair {
key: String,
value: f32,
}

type OrderedMap = Vec<Pair>;

fn less(p: &OrderedMap, i: usize, j: usize) -> bool {
p[i].value < p[j].value
}

