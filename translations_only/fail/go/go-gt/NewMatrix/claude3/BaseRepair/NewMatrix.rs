
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn new_matrix(n: i64) -> Box<Matrix> {
    let mut a = Vec::with_capacity((n * n) as usize);
    a.resize((n * n) as usize, 0);
    Box::new(Matrix { n, a })
}
