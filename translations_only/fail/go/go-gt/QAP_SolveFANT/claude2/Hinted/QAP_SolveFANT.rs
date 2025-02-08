

use std::{fmt::Display, vec};

const INF: i64 = i64::MAX;

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
    
    fn copy(&mut self, w: &Vector) {
        self.0.copy_from_slice(&w.0);
    }
    
    fn swap(&mut self, i: i64, j: i64) {
        self.0.swap(i as usize, j as usize);
    }
    
    fn print(&self) {
        for i in &self.0 {
            print!("{} ", i);
        }
        println!("");
    }
}

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Self {
            n,
            a: vec![0; (n * n) as usize].into_boxed_slice(),
        }
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[((i * self.n) + j) as usize] = v;
    }
    
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[((i * self.n) + j) as usize]
    }
}

fn qap_solve_fant(a: &Matrix, b: &Matrix, p: Vector, r: i64, m: i64) -> i64 {
   0
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
  
}

fn gen_trace(p: &Vector, trace: &Matrix) {
  
}

fn perm(p: &mut Vector) {
  
} 

fn unif(low: i64, high: i64) -> i64 {
  0  
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
  0
}

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vector, cost: &mut i64) {
  
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
  0
}

fn update_trace(n: i64, p: &Vector, best_p: &Vector, inc: &mut i64, r: i64, trace: &mut Matrix) {
  
}

