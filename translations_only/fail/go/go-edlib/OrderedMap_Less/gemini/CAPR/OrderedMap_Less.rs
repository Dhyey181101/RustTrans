
use std::cmp::Ordering;

#[derive(Debug)]
struct Pair {
    key: String,
    value: f32,
}

impl Pair {
    fn new(key: String, value: f32) -> Self {
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

fn less(i: usize, j: usize, pairs: &[Pair]) -> bool {
    pairs[i].value < pairs[j].value
}
