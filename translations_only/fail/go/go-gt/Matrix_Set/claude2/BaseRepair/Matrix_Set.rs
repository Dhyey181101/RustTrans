
use std::ops::IndexMut;

struct Matrix {
    n: usize,
    a: Box<[i64]>,
}

fn new_matrix(n: usize) -> Matrix {
    Matrix {
        n,
        a: Box::new([0; 100]),
    }
}

fn index_mut(m: &mut Matrix, index: usize) -> &mut i64 {
    &mut m.a[index]
}

fn index_mut_matrix(m: &mut Matrix, i: usize, j: usize) -> &mut i64 {
    let index = i * m.n + j;
    index_mut(m, index)  
}

fn set(m: &mut Matrix, i: usize, j: usize, v: i64) {
    let value = index_mut_matrix(m, i, j);
    *value = v;
}

