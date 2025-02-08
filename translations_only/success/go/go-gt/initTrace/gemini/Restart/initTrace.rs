
fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}

impl Matrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}
