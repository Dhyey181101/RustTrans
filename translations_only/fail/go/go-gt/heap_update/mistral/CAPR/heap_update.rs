

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
        for _ in 0..g.n {
            if g.get(i, j as i64) > 0 {
                if self.w[i as usize] + g.get(i, j as i64) < self.w[j as usize] {
                    p[j as usize] = i + 1;
                    self.w[j as usize] = self.w[i as usize] + g.get(i, j as i64);
                    self.up(j as usize);
                }
            }
            j += 1;
        }
    }

    fn up(&mut self, mut j: usize) {
        let mut i = (j - 1) / 2;
        while i != j && self.less(i, j) {
            self.swap(i, j);
            j = i;
            i = (j - 1) / 2;
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let temp = self.a[self.i[a]];
        self.a[self.i[a]] = self.a[self.i[b]];
        self.a[self.i[b]] = temp;
        let temp2 = self.i[a];
        self.i[a] = self.i[b];
        self.i[b] = temp2;
    }

    fn less(&self, a: usize, b: usize) -> bool {
        self.w[a] < self.w[b]
    }
}

