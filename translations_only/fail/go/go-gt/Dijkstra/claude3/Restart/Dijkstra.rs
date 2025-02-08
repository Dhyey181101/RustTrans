
use std::cmp::Ordering;

const MAX_INT: i64 = std::i64::MAX;

fn dijkstra(g: Box<Matrix>, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut h = new_heap(g.n);
    h.w[i as usize] = 0;
    swap(&mut h, i as usize, 0);
    while h.n > 0 {
        let i = pop(&mut h);
        if h.w[i] == MAX_INT {
            return p;
        }
        update(&mut h, &mut p, &g, i as i64);
    }
    p
}

fn new_heap(n: i64) -> Box<Heap> {
    let mut h = Box::new(Heap {
        n,
        i: vec![0; n as usize],
        a: vec![0; n as usize],
        w: vec![MAX_INT; n as usize],
    });
    for i in 0..n as usize {
        h.i[i] = i as i64;
        h.a[i] = i as i64;
    }
    h
}

fn swap(h: &mut Box<Heap>, a: usize, b: usize) {
    let i = h.i[a];
    let j = h.i[b];
    h.i.swap(a, b);
    h.a[i as usize] = b as i64;
    h.a[j as usize] = a as i64;
}

fn pop(h: &mut Box<Heap>) -> usize {
    let i = h.i[0] as usize;
    h.n -= 1;
    swap(h, 0, h.n as usize);
    down(h, 0);
    i
}

fn down(h: &mut Box<Heap>, i: usize) {
    let mut i = i;
    loop {
        let left = 2 * i + 1;
        if left >= h.n as usize {
            break;
        }
        let mut j = left;
        if let Some(right) = (left + 1).checked_sub(h.n as usize) {
            if !less(h, left, right) {
                j = right;
            }
        }
        if less(h, i, j) {
            break;
        }
        swap(h, i, j);
        i = j;
    }
}

fn less(h: &Box<Heap>, a: usize, b: usize) -> bool {
    h.w[h.i[a] as usize] < h.w[h.i[b] as usize]
}

fn update(h: &mut Box<Heap>, p: &mut Vec<i64>, g: &Matrix, i: i64) {
    let i = i as usize;
    for j in 0..g.n as usize {
        if g.a[i * g.n as usize + j] > 0 {
            let new_weight = h.w[i] + g.a[i * g.n as usize + j];
            if new_weight < h.w[j] {
                p[j] = (i as i64) + 1;
                h.w[j] = new_weight;
                up(h, h.a[j] as usize);
            }
        }
    }
}

fn up(h: &mut Box<Heap>, mut j: usize) {
    loop {
        let i = (j - 1) / 2;
        if i == j || less(h, i, j) {
            break;
        }
        swap(h, i, j);
        j = i;
    }
}

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
