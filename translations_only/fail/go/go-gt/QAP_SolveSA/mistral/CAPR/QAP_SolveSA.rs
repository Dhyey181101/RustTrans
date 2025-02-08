
use rand::Rng;
use std::fmt;

pub static Verbose: bool = false;

type Vector = Vec<i64>;
type Matrix = Vec<i64>;

struct MyMatrix {
    n: i64,
    a: Matrix,
}

impl MyMatrix {
    fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * (self.n as usize) + j]
    }
}

fn qap_solve_sa(a: &MyMatrix, b: &MyMatrix, p: Vector, m: i64) -> i64 {
    0
}
