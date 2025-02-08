

use std::boxed::Box;

pub struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

pub fn new_matrix(n: i64) -> Box<Matrix> {
    let mut a = vec![0; (n * n) as usize];
    let a = a.into_boxed_slice();
    Box::new(Matrix { n, a })
}

