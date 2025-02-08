
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Pair>;

fn len(p: &OrderedMap) -> usize {
    p.len()
}

