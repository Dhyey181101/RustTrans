
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn set(matrix: &mut Matrix, i: i64, j: i64, v: i64) {
    let index = (i * matrix.n + j) as usize;
    matrix.a[index] = v;
}
