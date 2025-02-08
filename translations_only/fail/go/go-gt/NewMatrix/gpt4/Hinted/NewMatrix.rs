
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn new_matrix(n: i64) -> Box<Matrix> {
    Box::new(Matrix {
        n,
        a: vec![0; (n * n) as usize],
    })
}
