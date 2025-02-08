
// OrderedMap is a Vec of pair type with String keys and f32 values.
// It implements sorting methods by values.
struct OrderedMap(Vec<pair>);

struct pair {
    Key: String,
    Value: f32,
}

fn len(p: &OrderedMap) -> isize {
    p.0.len() as isize
}
