
struct Pair {
    key: String,
    value: f32,
}

struct OrderedMap(Vec<Box<Pair>>);

impl OrderedMap {
    fn new() -> Self {
        OrderedMap(Vec::new())
    }

    fn len(&self) -> isize {
        self.0.len() as isize
    }
}

fn to_array(ordered_map: &OrderedMap) -> Vec<String> {
    let map_size = ordered_map.len() as usize;
    let mut arr = Vec::with_capacity(map_size);
    for pair in ordered_map.0.iter() {
        arr.push(pair.key.clone());
    }
    arr
}

fn len(ordered_map: &OrderedMap) -> isize {
    ordered_map.len()
}
