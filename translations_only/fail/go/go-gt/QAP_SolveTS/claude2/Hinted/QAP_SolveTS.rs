
use std::{fmt::Display, cmp::Ordering};

const INF: i64 = i64::MAX;

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
    
    fn copy(&mut self, other: &Vector) {
        self.0.copy_from_slice(&other.0);
    }
    
    fn swap(&mut self, i: i64, j: i64) {
        self.0.swap(i as usize, j as usize);
    }
    
    fn print(&self) {
        println!("{:?}", &self.0);
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Self {
            n,
            a: vec![0; (n * n) as usize],
        }
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

fn qap_solve_ts(
    a: &Matrix,
    b: &Matrix,
    p: &mut Vector,
    opt: i64,
    tabu_duration: i64,
    aspiration: i64,
    nr_iterations: i64,
) -> i64 {
    let mut best_cost = INF;
    let n = p.len();
    
    // Implementation
    
    best_cost  
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    // Implementation
    0
}

// Other functions

