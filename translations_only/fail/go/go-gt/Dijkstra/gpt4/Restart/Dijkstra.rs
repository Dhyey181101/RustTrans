
use std::i64;

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

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut h = new_heap(g.n);
    h.w[i as usize] = 0;
    swap(&mut h, i, 0);
    while h.n > 0 {
        let i = pop(&mut h);
        if h.w[i as usize] == i64::MAX {
            return p;
        }
        update(&mut h, &mut p, i, g);
    }
    p
}

fn new_heap(n: i64) -> Heap {
    Heap {
        n,
        i: (0..n).collect(),
        a: (0..n).collect(),
        w: vec![i64::MAX; n as usize],
    }
}

fn swap(h: &mut Heap, a: i64, b: i64) {
    let (i, j) = (h.i[a as usize], h.i[b as usize]);
    h.i.swap(a as usize, b as usize);
    h.a.swap(i as usize, j as usize);
}

fn pop(h: &mut Heap) -> i64 {
    let i = h.i[0];
    h.n -= 1;
    swap(h, 0, h.n);
    down(h, 0);
    i
}

fn down(h: &mut Heap, mut i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let mut j = left;
        let right = left + 1;
        if right < h.n && !less(h, left, right) {
            j = right;
        }
        if less(h, i, j) {
            break;
        }
        swap(h, i, j);
        i = j;
    }
}

fn less(h: &Heap, a: i64, b: i64) -> bool {
    let (i, j) = (h.i[a as usize], h.i[b as usize]);
    h.w[i as usize] < h.w[j as usize]
}

fn update(h: &mut Heap, p: &mut Vec<i64>, i: i64, g: &Matrix) {
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

fn up(h: &mut Heap, mut j: i64) {
    while j > 0 {
        let i = (j - 1) / 2;
        if i == j || less(h, i, j) {
            break;
        }
        swap(h, i, j);
        j = i;
    }
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
