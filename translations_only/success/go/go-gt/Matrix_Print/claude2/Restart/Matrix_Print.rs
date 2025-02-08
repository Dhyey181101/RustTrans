
use std::fmt;

struct Matrix {
    n: i64,
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

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]  
}
