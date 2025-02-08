
use std::boxed::Box;

pub struct Pair {
    key: Box<String>,
    value: f32,
}

pub type OrderedMap = Vec<Pair>;

pub fn len(p: &OrderedMap) -> isize {
    p.len() as isize
}
