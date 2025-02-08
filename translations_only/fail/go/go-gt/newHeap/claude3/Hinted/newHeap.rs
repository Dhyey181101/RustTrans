

use std::i64;
use std::cmp;

fn new_heap(n: i64) -> Heap {
    let mut i = vec![0; n as usize].into_boxed_slice();
    let mut a = vec![0; n as usize].into_boxed_slice();
    let mut w = vec![std::i64::MAX; n as usize].into_boxed_slice();

    for j in 0..n as usize {
        i[j] = j as i64;
        a[j] = j as i64;
    }

    Heap {
        n,
        i,
        a,
        w,
    }
}

struct Heap {
    n: i64,
    i: Box<[i64]>,
    a: Box<[i64]>,
    w: Box<[i64]>,
}

