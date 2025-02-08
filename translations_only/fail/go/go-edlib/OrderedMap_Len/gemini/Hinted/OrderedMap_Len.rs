
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

#[derive(Debug)]
pub struct OrderedMap {
    pairs: Vec<Pair>,
}

impl OrderedMap {
    pub fn new() -> Self {
        OrderedMap { pairs: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    pub fn insert(&mut self, key: String, value: f32) {
        self.pairs.push(Pair::new(key, value));
    }

    pub fn sort_by_value(&mut self) {
        self.pairs.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap_or(Ordering::Equal));
    }
}
