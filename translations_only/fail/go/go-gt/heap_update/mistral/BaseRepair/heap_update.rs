

use std::cmp::Ordering;

struct Heap {
    n: i64,
    i: Vec<usize>,
    a: Vec<i64>,
    w: Vec<i64>,
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

impl Heap {
    fn update(&mut self, p: &mut Vec<i64>, i: i64, g: &Matrix) {
        let mut j = 0;
        for j in 0..g.n {
            if g.get(i, j) > 0 {
                if self.w[i as usize] + g.get(i, j) < self.w[j as usize] {
                    p[j as usize] = i + 1;
                    self.w[j as usize] = self.w[i as usize] + g.get(i, j);
                    self.up(j as usize);
                }
            }
        }
    }

    fn up(&mut self, mut j: usize) {
        let mut i = (j as i64 - 1) as usize / 2;
        while i != j && self.less(i, j) {
            self.swap(i, j);
            j = i;
            i = (j as i64 - 1) as usize / 2;
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let i = self.i[a];
        let j = self.i[b];
        self.i[a] = self.i[b];
        self.i[b] = i;
        let temp = self.a[i];
        self.a[i] = self.a[j];
        self.a[j] = temp;
    }

    fn less(&self, a: usize, b: usize) -> bool {
        self.w[a] < self.w[b]
    }
}

