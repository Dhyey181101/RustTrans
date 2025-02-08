
use std::mem;

struct Heap {
    n: i64,
    i: Box<[i64]>,
    a: Box<[i64]>,
    w: Box<[i64]>,
}

fn down(h: &mut Heap, i: i64) {
    let mut i = i;
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let mut j = left;
        if left + 1 < h.n && !less(h, left, left + 1) {
            j = left + 1;
        }
        if less(h, i, j) {
            break;
        }
        swap(h, i, j);
        i = j;
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i[a as usize] = h.i[b as usize];
    h.i[b as usize] = i;
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}
