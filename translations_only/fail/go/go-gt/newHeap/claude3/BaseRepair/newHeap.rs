

use std::i64;
use std::cmp::Ordering;

fn new_heap(n: i64) -> Heap {
    let mut i = Vec::with_capacity(n as usize);
    let mut a = Vec::with_capacity(n as usize);
    let mut w = Vec::with_capacity(n as usize);

    for j in 0..n as usize {
        i.push(j as i64);
        a.push(j as i64);
        w.push(std::i64::MAX);
    }

    Heap {
        n,
        i: i.into_boxed_slice(),
        a: a.into_boxed_slice(),
        w: w.into_boxed_slice(),
    }
}

struct Heap {
    n: i64,
    i: Box<[i64]>,
    a: Box<[i64]>,
    w: Box<[i64]>,
}

