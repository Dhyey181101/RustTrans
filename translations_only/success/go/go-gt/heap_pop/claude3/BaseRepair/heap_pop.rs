
use std::mem;

struct Heap {
    n: usize,
    i: Vec<usize>,
    a: Vec<usize>,
    w: Vec<usize>,
}

fn pop(h: &mut Box<Heap>) -> usize {
    let i = h.i[0];
    h.n -= 1;
    swap(h, 0, h.n);
    down(h, 0);
    i
}

fn swap(h: &mut Box<Heap>, a: usize, b: usize) {
    let i = h.i[a];
    let j = h.i[b];
    h.i.swap(a, b);
    h.a[i] = b;
    h.a[j] = a;
}

fn down(h: &mut Box<Heap>, mut i: usize) {
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

fn less(h: &Box<Heap>, a: usize, b: usize) -> bool {
    h.w[h.i[a]] < h.w[h.i[b]]
}
