
use std::mem;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        let idx = i as usize * self.n as usize + j as usize;
        mem::replace(&mut self.a[idx], v);
    }
}
