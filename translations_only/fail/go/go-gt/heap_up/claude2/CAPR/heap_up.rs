
use std::mem;

struct Heap {
    i: Vec<usize>,
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
    let i = h.a[h.i[a]];
    let j = h.a[h.i[b]];
    h.a[h.i[a]] = j;
    h.a[h.i[b]] = i;
}

fn less(h: &Heap, a: usize, b: usize) -> bool {
    let i = h.i[a];
    let j = h.i[b];
    h.w[i] < h.w[j]  
}

