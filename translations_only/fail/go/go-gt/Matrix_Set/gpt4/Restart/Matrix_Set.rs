
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Self {
            n,
            a: vec![0; (n * n) as usize].into_boxed_slice(),
        }
    }
}

fn set(matrix: &mut Matrix, i: i64, j: i64, v: i64) {
    let index = (i * matrix.n + j) as usize;
    matrix.a[index] = v;
}
