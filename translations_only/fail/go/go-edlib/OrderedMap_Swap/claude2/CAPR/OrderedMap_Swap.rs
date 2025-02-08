
use std::mem;

#[derive(Clone)]
struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Pair>;

pub fn swap(v: &mut OrderedMap, i: isize, j: isize) {
    let len = v.len();
    if i >= 0 && i < len as isize && j >= 0 && j < len as isize {
        let i = i as usize;
        let j = j as usize;
        let temp = v[i].clone();
        let ith = mem::replace(&mut v[i], temp);
        let jth = mem::replace(&mut v[j], ith);
        *v.get_mut(j).unwrap() = jth; 
    }
}

