
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Pair>;

fn swap(ordered_map: &mut OrderedMap, i: isize, j: isize) {
    let len = ordered_map.len() as isize;
    if i >= 0 && j >= 0 && i < len && j < len {
        ordered_map.swap(i as usize, j as usize);
    }
}
