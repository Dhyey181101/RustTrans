
use std::boxed::Box;

fn less(h: &Box<Heap>, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}

struct Heap {
    i: Box<[i64]>,
    w: Box<[i64]>,
}
