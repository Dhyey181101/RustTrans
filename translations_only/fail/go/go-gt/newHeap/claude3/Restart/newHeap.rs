
use std::i64;

fn new_heap(n: usize) -> Heap {
    let mut i = vec![0; n].into_boxed_slice();
    let mut a = vec![0; n].into_boxed_slice();
    let mut w = vec![i64::MAX; n].into_boxed_slice();

    for idx in 0..n {
        i[idx] = idx as i64;
        a[idx] = idx as i64;
    }

    Heap {
        n: n as i64,
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
