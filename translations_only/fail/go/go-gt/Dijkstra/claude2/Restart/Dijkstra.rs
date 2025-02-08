

use std::cmp::Ordering;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut h = new_heap(g.n);
    h.w[i as usize] = 0;
    swap(&mut h, i, 0);
    while h.n > 0 {
        let i = pop(&mut h);
        if h.w[i as usize] == std::i64::MAX {
            return p;
        }
        update(&mut p, i, g, &mut h);
    }
    p  
}

fn new_heap(n: i64) -> Heap {
    let mut h = Heap {
        n,
        i: vec![0; n as usize],
        a: vec![0; n as usize], 
        w: vec![std::i64::MAX; n as usize],
    };
    for i in 0..n {
        h.i[i as usize] = i;
        h.a[i as usize] = i;
    }
    h
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.i[a as usize] = j;
    h.i[b as usize] = i;
    h.a[i as usize] = b;
    h.a[j as usize] = a;  
}

fn pop(h: &mut Heap) -> i64 {
    let i = h.i[0];
    h.n -= 1; 
    swap(h, 0, h.n);
    down(h, 0);
    i
}

fn down(h: &mut Heap, i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let j = if left + 1 < h.n && less(h, left, left + 1) {
            left + 1
        } else {
            left
        };
        if !less(h, i, j) {
            break;
        }
        swap(h, i, j);
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let i = h.i[a as usize];
    let j = h.i[b as usize];
    h.w[i as usize] < h.w[j as usize]  
}

fn update(p: &mut Vec<i64>, i: i64, g: &Matrix, h: &mut Heap) {
    for j in 0..g.n {
        if g.get(i, j) > 0 {
            if h.w[i as usize] + g.get(i, j) < h.w[j as usize] {
                p[j as usize] = i + 1;
                h.w[j as usize] = h.w[i as usize] + g.get(i, j);
                up(h, h.a[j as usize]);
            }
        }
    }
}

fn up(mut h: &mut Heap, mut j: i64) {
    loop {
        let i = (j - 1) / 2;
        if i == j || less(&h, i, j) { 
            break;
        }
        swap(&mut h, i, j);
        j = i;
    }
}

