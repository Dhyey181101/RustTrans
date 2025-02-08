
use std::i64::MAX;

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn new_heap(n: i64) -> Box<Heap> {
    let mut heap = Heap {
        n,
        i: Vec::with_capacity(n as usize),
        a: Vec::with_capacity(n as usize),
        w: Vec::with_capacity(n as usize),
    };

    for index in 0..n {
        heap.i.push(index);
        heap.a.push(index);
        heap.w.push(MAX);
    }

    Box::new(heap)
}
