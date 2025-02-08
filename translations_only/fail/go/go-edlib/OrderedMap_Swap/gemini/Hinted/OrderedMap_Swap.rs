
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Pair {
    pub key: String,
    pub value: f32,
}

impl Pair {
    pub fn new(key: String, value: f32) -> Self {
        Self { key, value }
    }
}

pub fn swap(p: &mut [Pair], i: usize, j: usize) {
    p.swap(i, j);
}

pub fn is_valid(p: &[Pair]) -> bool {
    p.iter().all(|pair| pair.value.is_finite())
}

pub fn sort_by_value(p: &mut [Pair]) {
    p.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(Ordering::Equal));
}
