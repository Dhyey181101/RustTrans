
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut h = Heap::new(g.n);
    h.w[i as usize] = 0;
    h.swap(i as usize, 0);
    while h.n > 0 {
        let i = h.pop();
        if h.w[i as usize] == i64::MAX {
            return p;
        }
        h.update(&mut p, i, g);
    }
    p
}

struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn new(n: i64) -> Self {
        Self {
            n,
            i: vec![0; n as usize],
            a: vec![0; n as usize],
            w: vec![i64::MAX; n as usize],
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let i = self.i[a];
        let j = self.i[b];
        self.i[a] = j;
        self.i[b] = i;
        self.a[i as usize] = b as i64;
        self.a[j as usize] = a as i64;
    }

    fn pop(&mut self) -> i64 {
        let i = self.i[0];
        self.n -= 1;
        self.swap(0, self.n as usize);
        self.down(0);
        i
    }

    fn down(&mut self, mut i: usize) {
        loop {
            let left = 2 * i + 1;
            if left >= self.n as usize {
                break;
            }
            let mut j = left;
            if let Some(right) = left.checked_add(1) {
                if right < self.n as usize && !self.less(left, right) {
                    j = right;
                }
            }
            if self.less(i, j) {
                break;
            }
            self.swap(i, j);
            i = j;
        }
    }

    fn less(&self, a: usize, b: usize) -> bool {
        let i = self.i[a];
        let j = self.i[b];
        self.w[i as usize] < self.w[j as usize]
    }

    fn update(&mut self, p: &mut Vec<i64>, i: i64, g: &Matrix) {
        let mut j = 0;
        while j < g.n {
            if g.get(i, j) > 0 {
                if self.w[i as usize] + g.get(i, j) < self.w[j as usize] {
                    p[j as usize] = i + 1;
                    self.w[j as usize] = self.w[i as usize] + g.get(i, j);
                    self.up(self.a[j as usize] as usize);
                }
            }
            j += 1;
        }
    }

    fn up(&mut self, mut j: usize) {
        loop {
            let i = (j - 1) / 2;
            if i == j || self.less(i, j) {
                break;
            }
            self.swap(i, j);
            j = i;
        }
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
