
use std::mem;

struct Heap {
    i: Box<[i64]>,
    a: Box<[i64]>,
    w: Box<[i64]>,
}

fn up(h: &mut Heap, j: i64) {
    let mut j = j;
    loop {
        let i = (j - 1) / 2;
        if i == j || less(h, i, j) {
            break;
        }
        swap(h, i, j);
        j = i;
    }
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i[a as usize] = j;
    h.i[b as usize] = i;
    h.a[i as usize] = b;
    h.a[j as usize] = a;
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}
