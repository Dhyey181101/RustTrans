
use std::collections::HashMap;

fn to_array(p: &HashMap<String, f32>) -> Vec<String> {
    let map_size = p.len();
    let mut arr = Vec::with_capacity(map_size);
    for (key, _) in p {
        arr.push(key.clone());
    }

    arr
}

fn len(p: &HashMap<String, f32>) -> usize {
    p.len()
}

#[derive(Debug)]
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Pair>;

fn main() {
    let mut p = HashMap::new();
    p.insert("".to_string(), 1.8737771e-38);
    println!("{:?}", to_array(&p));

    p.clear();
    p.insert("".to_string(), 2.4074124e-33);
    println!("{:?}", to_array(&p));
}
