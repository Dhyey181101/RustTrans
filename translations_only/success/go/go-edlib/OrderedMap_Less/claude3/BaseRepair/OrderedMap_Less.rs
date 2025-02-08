
use std::cmp::Ordering;

const _: () = ();

fn less(p: &OrderedMap, i: isize, j: isize) -> bool {
    p[i as usize].value < p[j as usize].value
}

#[derive(Clone)]
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Box<Pair>>;
