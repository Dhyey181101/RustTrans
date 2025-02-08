
use std::mem;

struct Pair {
    key: String,
    value: f32,
}

type OrderedMap = Vec<Pair>;

fn swap(p: &mut OrderedMap, i: isize, j: isize) {
    if i < 0 || j < 0 {
        panic!("Invalid index");
    }
    if i as usize >= p.len() || j as usize >= p.len() {
        panic!("Index out of bounds");
    }
    unsafe {
        let ptr_i = p.as_mut_ptr().add(i as usize);
        let ptr_j = p.as_mut_ptr().add(j as usize);
        mem::swap(&mut *ptr_i, &mut *ptr_j); 
    }
}
