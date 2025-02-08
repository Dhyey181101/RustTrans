
use std::mem;

struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn up(mut h: Box<Heap>, mut j: usize) {
    loop {
        let i = (j - 1) / 2;
        if i == j || less(&h, i, j) {
            break;
        }
        swap(&mut h, i, j);
        j = i;
    }
}

fn swap(h: &mut Box<Heap>, a: usize, b: usize) {
    let tmp = h.i[a];
    h.i[a] = h.i[b];
    h.i[b] = tmp;
    let i = h.a[h.i[a] as usize];
    let j = h.a[h.i[b] as usize];
    h.a[h.i[a] as usize] = j;
    h.a[h.i[b] as usize] = i;
}

fn less(h: &Heap, a: usize, b: usize) -> bool {
    let i = h.i[a];
    let j = h.i[b];
    h.w[i as usize] < h.w[j as usize]
}

