
use std::boxed::Box;

struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i_a = h.i[a as usize];
    let i_b = h.i[b as usize];
    h.i[a as usize] = i_b;
    h.i[b as usize] = i_a;
    h.a[i_a as usize] = b;
    h.a[i_b as usize] = a;
}
