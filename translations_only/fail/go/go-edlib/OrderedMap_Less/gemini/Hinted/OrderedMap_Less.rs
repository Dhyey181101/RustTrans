
use std::cmp::Ordering;

#[derive(Debug)]
pub struct Pair {
    pub key: String,
    pub value: f32,
}

impl Pair {
    pub fn new(key: String, value: f32) -> Self {
        Pair { key, value }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub fn less(i: usize, j: usize, pairs: &[Pair]) -> bool {
    pairs[i].value < pairs[j].value
}
