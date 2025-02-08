
use std::mem;

struct Env {
    n: i64,
    g: Matrix,

    s: Vec<bool>,

    slack: Vec<i64>,
    slackx: Vec<i64>,
    prev: Vec<i64>,

    lx: Vec<i64>,
    ly: Vec<i64>,
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

impl Env {
    fn add(&mut self, i: i64, p: i64) {
        let mut j = 0;
        self.s[i as usize] = true;
        self.prev[i as usize] = p;
        for _ in 0..self.n {
            if self.lx[i as usize] + self.ly[i as usize] - self.g.get(i, j) < self.slack[i as usize] {
                self.slack[i as usize] = self.lx[i as usize] + self.ly[i as usize] - self.g.get(i, j);
                self.slackx[i as usize] = j;
            }
            j += 1;
        }
    }
}
