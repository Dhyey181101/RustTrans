
pub struct OrderedMap {
    pairs: Vec<Pair>,
}

pub struct Pair {
    key: String,
    value: f32,
}

impl OrderedMap {
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
}
