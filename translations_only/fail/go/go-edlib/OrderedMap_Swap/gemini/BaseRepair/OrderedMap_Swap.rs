
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Pair {
    pub key: String,
    pub value: f32,
}

impl Pair {
    pub fn new(key: String, value: f32) -> Self {
        Self { key, value }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Pair {}

pub fn swap(p: &mut [Pair], i: usize, j: usize) {
    p.swap(i, j);
}

pub fn ordered_map(p: &mut [Pair]) {
    p.sort_by(|a, b| a.cmp(b));
}
