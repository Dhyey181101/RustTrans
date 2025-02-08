
use std::mem;

struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Box<Pair>>;

pub fn swap(v: &mut OrderedMap, i: isize, j: isize) {
    let len = v.len();
    if i >= 0 && i < len as isize && j >= 0 && j < len as isize {
        let i_pair = mem::replace(&mut v[i as usize], Box::new(Pair {
            key: String::new(),
            value: 0.0    
        }));
        let j_pair = mem::replace(&mut v[j as usize], Box::new(Pair {
            key: String::new(),
            value: 0.0
        }));
        *v[j as usize] = *i_pair;
        *v[i as usize] = *j_pair;
    }
}

