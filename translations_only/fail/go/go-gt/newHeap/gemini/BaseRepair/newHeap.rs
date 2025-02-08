
use std::i64;

fn new_heap(n: i64) -> Heap {
    Heap {
        N: n,
        I: vec![0; n as usize],
        A: vec![0; n as usize],
        W: vec![i64::MAX; n as usize],
    }
}

struct Heap {
    N: i64,
    I: Vec<i64>,
    A: Vec<i64>,
    W: Vec<i64>,
}
