
use std::boxed::Box;

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Matrix {
            n,
            a: vec![0; (n * n) as usize].into_boxed_slice(),
        }
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}
