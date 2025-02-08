
use std::mem;

type OrderedMap = Vec<Pair>;

#[derive(Clone)]
struct Pair {
    key: String,
    value: f32,
}

fn swap(p: &mut OrderedMap, i: isize, j: isize) {
    let temp = p[i as usize].clone();
    p[i as usize] = mem::replace(&mut p[j as usize], temp);
}
