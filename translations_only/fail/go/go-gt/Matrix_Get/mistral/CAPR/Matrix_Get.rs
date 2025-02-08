
use std::boxed::Box;

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
