
use std::mem;

struct Heap {
    n: i64,
    i: Vec<usize>,
    a: Vec<usize>,
    w: Vec<i64>,
}

fn pop(h: &mut Heap) -> i64 {
    let i = h.i[0];
    h.n -= 1;
    swap(h, 0, h.n as usize);
    down(h, 0);
    i as i64
}

fn swap(h: &mut Heap, a: usize, b: usize) {
    let tmp = h.i[a];
    h.i[a] = h.i[b];
    h.i[b] = tmp;
    let i = h.i[a];
    let j = h.i[b];
    h.a[i] = b as usize;
    h.a[j] = a as usize;
}

fn down(mut h: &mut Heap, mut i: usize) {
    loop {
        let left = 2 * i + 1;
        if left >= h.n as usize {
            break;
        }
        let mut j = left;
        if left + 1 < h.n as usize && !less(&h, left, (left + 1) as usize) {
            j = left + 1;
        }
        if less(&h, i, j) {
            break;
        }
        swap(&mut h, i, j);
        i = j;
    }
}

fn less(h: &Heap, a: usize, b: usize) -> bool {
    let i = h.i[a];
    let j = h.i[b];
    h.w[i] < h.w[j]
}

