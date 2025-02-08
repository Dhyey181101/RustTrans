

use std::collections::BinaryHeap;
use std::usize;

const INF: i64 = i64::MAX;

type Matrix = Vec<Vec<i64>>;

struct Heap {
    n: usize,
    i: Vec<usize>,
    a: Vec<usize>,
    w: Vec<i64>,
}

impl Heap {
    fn new(n: usize) -> Self {
        let mut i = (0..n).collect::<Vec<_>>();
        let a = i.clone();
        let w = vec![INF; n];
        Heap { n, i, a, w }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let i_a = self.i[a];
        let i_b = self.i[b];
        self.i[a] = i_b;
        self.i[b] = i_a;
        self.a[i_a] = b;
        self.a[i_b] = a;
    }

    fn pop(&mut self) -> usize {
        let n = self.n;
        self.swap(0, n - 1);
        self.n -= 1;
        self.down(0, n);
        self.i[n]
    }

    fn down(&mut self, i: usize, n: usize) {
        let mut i = i;
        loop {
            let left = 2 * i + 1;
            if left >= n {
                break;
            }
            let j = if left + 1 < n && self.w[left] > self.w[left + 1] {
                left + 1
            } else {
                left
            };
            if self.w[j] < self.w[i] {
                self.swap(i, j);
                i = j;
            } else {
                break;
            }
        }
    }
}

fn dijkstra(g: &Matrix, s: usize) -> Matrix {
    let n = g.len();
    let mut h = Heap::new(n);
    let mut d = vec![vec![INF; n]; n];
    d[s][s] = 0;
    h.w[s] = 0;
    while h.n > 0 {
        let v = h.pop();
        for i in 0..g[v].len() {
            let w = g[v][i];
            if w > 0 && d[s][v] + w < d[s][i] {
                d[s][i] = d[s][v] + w;
                h.w[h.i[i]] = d[s][i];
                h.down(h.i[i], h.n);
            }
        }
    }
    d
}

