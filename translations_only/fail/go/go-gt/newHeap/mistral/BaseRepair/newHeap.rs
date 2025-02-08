

use std::mem;

type Int64 = i64;
type Heap = Box<HeapStruct>;

struct HeapStruct {
    n: Int64,
    i: Vec<Int64>,
    a: Vec<Int64>,
    w: Vec<Int64>,
}

const MAX_INT64: Int64 = i64::MAX;

fn new_heap(n: Int64) -> Heap {
    let mut h = Box::new(HeapStruct {
        n: n,
        i: Vec::with_capacity(n as usize),
        a: Vec::with_capacity(n as usize),
        w: Vec::with_capacity(n as usize),
    });
    h.i.resize(n as usize, 0);
    h.a.resize(n as usize, 0);
    h.w.resize(n as usize, MAX_INT64);
    h
}

