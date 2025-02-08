
use std::boxed::Box;

struct Matrix {
    n: usize,
    a: Box<[i64]>,
}

fn new_matrix(n: usize) -> Box<Matrix> {
    let a = vec![0; n * n].into_boxed_slice();

    Box::new(Matrix {
        n,
        a,
    })
}
