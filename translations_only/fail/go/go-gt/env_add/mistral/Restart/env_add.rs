

extern crate libc;

use libc::int64_t;
use std::boxed::Box;

struct Env {
    n: int64_t,
    g: Box<Matrix>,

    s: Vec<bool>,

    slack: Vec<int64_t>,
    slackx: Vec<int64_t>,
    prev: Vec<int64_t>,

    lx: Vec<int64_t>,
    ly: Vec<int64_t>,
}

struct Matrix {
    n: int64_t,
    a: Vec<int64_t>,
}

impl Matrix {
    fn get(&self, i: int64_t, j: int64_t) -> int64_t {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

impl Env {
    fn add(&mut self, i: int64_t, p: int64_t) {
        let mut j: int64_t = 0;
        self.s[i as usize] = true;
        self.prev[i as usize] = p;
        for j in 0..self.n {
            if self.lx[i as usize] + self.ly[i as usize] - self.g.get(i, j) < self.slack[i as usize] {
                self.slack[i as usize] = self.lx[i as usize] + self.ly[i as usize] - self.g.get(i, j);
                self.slackx[i as usize] = j;
            }
        }
    }
}

fn main() {}

