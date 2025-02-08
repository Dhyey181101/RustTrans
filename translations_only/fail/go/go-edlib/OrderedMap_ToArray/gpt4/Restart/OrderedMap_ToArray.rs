
struct Pair {
    key: String,
    value: f32,
}

struct OrderedMap(Vec<Box<Pair>>);

impl OrderedMap {
    fn new() -> Self {
        OrderedMap(Vec::new())
    }

    fn to_array(&self) -> Vec<String> {
        self.0.iter().map(|pair| pair.key.clone()).collect()
    }

    fn len(&self) -> isize {
        self.0.len() as isize
    }
}

fn main() {}
