
use rand::Rng;
use std::fmt;

pub static Verbose: bool = false;

type Vector = Vec<i64>;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut sum: i64 = 0;
    for i in 0..a.n {
        for j in 0..a.n {
            let diff: i64 = a.get(i, j) - b.get(i, j);
            sum += diff * diff * p[i as usize] * p[j as usize];
        }
    }
    sum
}
