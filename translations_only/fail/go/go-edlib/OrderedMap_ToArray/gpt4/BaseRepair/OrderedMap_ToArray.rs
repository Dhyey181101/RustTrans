
struct Pair {
    key: String,
    value: f32,
}

struct OrderedMap(Vec<Pair>);

impl OrderedMap {
    fn new(pairs: Vec<Pair>) -> Self {
        OrderedMap(pairs)
    }

    fn len(&self) -> isize {
        self.0.len() as isize
    }
}

fn to_array(ordered_map: &OrderedMap) -> Vec<String> {
    let map_size = ordered_map.len() as usize;
    let mut arr = Vec::with_capacity(map_size);
    for elem in &ordered_map.0 {
        arr.push(elem.key.clone());
    }
    arr
}

fn main() {
    let pairs = vec![
        Pair { key: "one".to_string(), value: 1.0 },
        Pair { key: "two".to_string(), value: 2.0 },
    ];
    let ordered_map = OrderedMap::new(pairs);
    let array = to_array(&ordered_map);
    println!("{:?}", array);
}
