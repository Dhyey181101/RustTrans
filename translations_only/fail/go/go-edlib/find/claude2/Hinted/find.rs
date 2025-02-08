
use std::cmp::Ordering;

fn find(slice: &[Box<[isize]>], val: &[isize]) -> isize {
    for (i, item) in slice.iter().enumerate() {
        if equal(item, val) {
            return i as isize;
        }
    }
    -1
}

fn equal(a: &[isize], b: &[isize]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        match a[i].cmp(&b[i]) {
            Ordering::Equal => continue,
            _ => return false,
        }
    }
    true
}

