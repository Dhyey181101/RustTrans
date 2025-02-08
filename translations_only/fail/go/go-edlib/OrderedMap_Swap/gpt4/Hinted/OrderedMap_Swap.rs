
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Pair>;

fn swap(ordered_map: &mut OrderedMap, i: isize, j: isize) {
    if i >= 0 && j >= 0 && i < ordered_map.len() as isize && j < ordered_map.len() as isize {
        ordered_map.swap(i as usize, j as usize);
    } else {
        panic!("Input is invalid, crash gracefully");
    }
}
