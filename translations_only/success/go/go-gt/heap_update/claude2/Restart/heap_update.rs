

use std::mem;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn update(h: &mut Heap, p: &mut [i64], i: i64, g: &Matrix) {
    let mut j = 0;
    while j < g.n {
        if get(g, i, j) > 0 {
            if h.w[i as usize] + get(g, i, j) < h.w[j as usize] {
                p[j as usize] = i + 1;
                h.w[j as usize] = h.w[i as usize] + get(g, i, j);
                up(h, h.a[j as usize]);
            }
        }
        j += 1;
    }
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    *m.a.get((i * m.n + j) as usize).unwrap()  
}

fn up(h: &mut Heap, mut j: i64) {
    let mut i;
    loop {
        i = (j - 1) / 2;
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
    h.i.swap(a as usize, b as usize);
    h.a.swap(i as usize, j as usize);
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize] 
}

