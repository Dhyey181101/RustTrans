
use std::mem;

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn pop(h: &mut Heap) -> i64 {
    let i = h.i[0 as usize];
    h.n -= 1;
    swap(h, 0, h.n as usize);
    down(h, 0);
    i
}

fn swap(h: &mut Heap, a: usize, b: usize) {
    let tmp = h.i[a];
    h.i[a] = h.i[b];
    h.i[b] = tmp;
    let i = h.i[a];
    let j = h.i[b];
    h.a[i as usize] = b as i64;
    h.a[j as usize] = a as i64;
}

fn down(mut h: &mut Heap, mut i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let mut j = left as usize;
        if left + 1 < h.n && !less(&h, j, (left + 1) as usize) {
            j = (left + 1) as usize;
        }
        if less(&h, i as usize, j) {
            break;
        }
        swap(&mut h, i as usize, j);
        i = j as i64;
    }
}

fn less(h: &Heap, a: usize, b: usize) -> bool {
    let i = h.i[a];
    let j = h.i[b];
    h.w[i as usize] < h.w[j as usize]
}


