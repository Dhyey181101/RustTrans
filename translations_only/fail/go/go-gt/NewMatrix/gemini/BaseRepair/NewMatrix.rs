
pub fn new_matrix(n: i64) -> Box<Matrix> {
    Box::new(Matrix {
        n,
        a: vec![0; (n * n) as usize],
    })
}

pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}
