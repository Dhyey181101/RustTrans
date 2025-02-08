
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        let index = (i * self.n + j) as usize;
        self.a[index] = v;
    }
}

fn main() {}
