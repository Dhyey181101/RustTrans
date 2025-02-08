
#[derive(Clone)]
struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn set(m: Box<Matrix>, i: i64, j: i64, v: i64) -> Box<Matrix> {
    let mut new_a: Vec<i64> = m.a.into_vec();
    new_a[(i * m.n + j) as usize] = v;
    Box::new(Matrix {
        n: m.n,
        a: new_a.into_boxed_slice(),
    })
}
