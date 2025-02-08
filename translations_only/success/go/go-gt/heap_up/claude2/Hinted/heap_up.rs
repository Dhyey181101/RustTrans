
use std::mem;

struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,  
    w: Vec<i64>,
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i.swap(a as usize, b as usize);
    h.a.swap(i as usize, j as usize); 
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]
}

fn up(h: &mut Heap, j: i64) {
    let mut j = j;
    loop {
        let i = (j - 1) / 2;
        if i == j || less(&h, i, j) {
            break;
        }
        swap(h, i, j);
        j = i;
    }
}
