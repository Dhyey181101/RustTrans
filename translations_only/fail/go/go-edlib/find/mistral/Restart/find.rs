

use std::boxed::Box;

fn find(slice: Vec<Box<[u8]>>, val: &Box<[u8]>) -> isize {
    for (i, item) in slice.into_iter().enumerate() {
        if equal(&item, val) {
            return i as isize;
        }
    }
    -1
}

fn equal(a: &Box<[u8]>, b: &Box<[u8]>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (i, v) in a.iter().enumerate() {
        if *v != (*b)[i] {
            return false;
        }
    }
    true
}

