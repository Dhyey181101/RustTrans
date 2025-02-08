
use std::boxed::Box;

#[derive(Debug)]
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Box<Pair>>;

fn len(p: &OrderedMap) -> isize {
    p.len() as isize
}
