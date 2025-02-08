
use std::boxed::Box;
use std::mem;
use std::vec::Vec;

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
        i: vec![0; n as usize],
        a: vec![0; n as usize],
        w: vec![MAX_INT64; n as usize],
    });
    unsafe {
        for i in 0..n {
            (*h).i[i as usize] = i;
            (*h).a[i as usize] = i;
            (*h).w[i as usize] = MAX_INT64;
        }
    }
    h
}
