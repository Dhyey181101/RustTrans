
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn get(matrix: &Matrix, i: i64, j: i64) -> i64 {
    matrix.a[(i * matrix.n + j) as usize]
}

