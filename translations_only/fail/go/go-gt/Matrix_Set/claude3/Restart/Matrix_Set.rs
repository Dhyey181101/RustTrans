
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn set(m: Box<Matrix>, i: i64, j: i64, v: i64) -> Box<Matrix> {
    let mut new_m = m;
    new_m.a[(i * new_m.n + j) as usize] = v;
    new_m
}
