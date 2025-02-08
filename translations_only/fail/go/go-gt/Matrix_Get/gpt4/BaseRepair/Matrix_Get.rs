
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn new(n: i64, a: Vec<i64>) -> Self {
        Matrix { n, a: a.into_boxed_slice() }
    }
}

fn get(matrix: &Matrix, i: i64, j: i64) -> i64 {
    matrix.a[(i * matrix.n + j) as usize]
}

fn main() {}
