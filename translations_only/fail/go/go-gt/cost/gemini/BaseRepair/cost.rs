
use std::boxed::Box;

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    for i in 0..p.0.len() {
        for j in 0..p.0.len() {
            c += a.get(i, j) * b.get(p.0[i] as usize, p.0[j] as usize);
        }
    }
    c
}

impl Matrix {
    fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * self.n + j]
    }
}

struct Vector(Vec<i64>);

struct Matrix {
    n: usize,
    a: Vec<i64>,
}
