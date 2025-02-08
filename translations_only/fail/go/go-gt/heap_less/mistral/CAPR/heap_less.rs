

use std::boxed::Box;

struct Heap {
 i: Vec<usize>,
 w: Vec<i64>,
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
 h.w[h.i[a as usize]] < h.w[h.i[b as usize]]
}

