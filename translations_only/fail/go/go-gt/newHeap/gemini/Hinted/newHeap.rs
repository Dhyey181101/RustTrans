
use std::i64;

pub fn new_heap(n: i64) -> Heap {
    Heap {
        n,
        i: vec![0; n as usize],
        a: vec![0; n as usize],
        w: vec![i64::MAX; n as usize],
    }
}

pub struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}
