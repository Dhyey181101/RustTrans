
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[((i * m.n) + j) as usize]
}
