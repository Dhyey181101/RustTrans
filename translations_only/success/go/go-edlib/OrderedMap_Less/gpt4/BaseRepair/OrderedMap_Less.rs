
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Box<Pair>>;

fn less(ordered_map: &OrderedMap, i: isize, j: isize) -> bool {
    ordered_map[i as usize].value < ordered_map[j as usize].value
}
