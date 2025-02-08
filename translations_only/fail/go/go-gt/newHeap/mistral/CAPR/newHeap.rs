
use std::i64;
use std::boxed::Box;

pub struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

pub fn new_heap(n: i64) -> Heap {
    Heap {
        n: n,
        i: vec![0; n as usize],
        a: vec![0; n as usize],
        w: vec![i64::MAX; n as usize],
    }
}
