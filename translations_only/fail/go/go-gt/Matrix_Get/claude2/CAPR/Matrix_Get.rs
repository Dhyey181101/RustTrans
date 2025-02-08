
use std::ops::IndexMut;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i as usize * m.n as usize + j as usize] 
}

