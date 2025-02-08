
use std::mem;

struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn up(mut h: Box<Heap>, mut j: i64) {
    loop {
        let i = (j - 1) / 2;
        if i == j || less(&h, i, j) {
            break;
        }
        swap(&mut h, i, j);
        j = i;
    }
}

fn swap(h: &mut Box<Heap>, a: i64, b: i64) {
    let tmp = h.i[a as usize];
    h.i[a as usize] = h.i[b as usize];
    h.i[b as usize] = tmp;
    let i = h.a[h.i[a as usize] as usize];
    let j = h.a[h.i[b as usize] as usize];
    h.a[h.i[a as usize] as usize] = j;
    h.a[h.i[b as usize] as usize] = i;  
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]  
}

