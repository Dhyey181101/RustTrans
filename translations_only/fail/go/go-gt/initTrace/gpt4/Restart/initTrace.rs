
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Self {
            n,
            a: vec![0; (n * n) as usize],
        }
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        let index = (i * self.n + j) as usize;
        self.a[index] = v;
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}

fn main() {
    let mut matrix = Matrix::new(5);
    init_trace(5, 1, &mut matrix);
}
