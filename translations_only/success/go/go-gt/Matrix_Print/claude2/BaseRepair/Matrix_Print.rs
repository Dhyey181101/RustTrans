
use std::fmt;

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

fn print(m: &Matrix) {
    for i in 0..m.n {
        for j in 0..m.n {
            print!("{} ", get(m, i, j));
        }
        println!("");
    }
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i * m.n + j] 
}

