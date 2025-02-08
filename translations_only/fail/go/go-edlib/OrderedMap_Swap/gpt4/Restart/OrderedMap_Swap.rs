
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Box<Pair>>;

fn swap(ordered_map: &mut OrderedMap, i: isize, j: isize) {
    let i_usize = i as usize;
    let j_usize = j as usize;
    ordered_map.swap(i_usize, j_usize);
}
