
fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    m.a[(i * m.n + j) as usize] = v;
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}
