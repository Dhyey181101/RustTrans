
use std::i64;

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn new_heap(n: i64) -> Box<Heap> {
    let mut h = Box::new(Heap {
        n,
        i: vec![0; n as usize],
        a: vec![0; n as usize],
        w: vec![i64::MAX; n as usize],
    });
    
    for i in 0..n {
        h.i[i as usize] = i;
        h.a[i as usize] = i;
    }
    
    h
}
