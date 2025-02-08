
use std::ops::IndexMut;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn new_matrix(n: i64) -> Box<Matrix> {
    Box::new(Matrix {
        n,
        a: vec![0; n as usize * n as usize],
    })
}

fn index_mut(m: &mut Matrix, index: usize) -> &mut i64 {
    &mut m.a[index]
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    let index = ((i * m.n + j) as usize);
    *index_mut(m, index) = v;
}
