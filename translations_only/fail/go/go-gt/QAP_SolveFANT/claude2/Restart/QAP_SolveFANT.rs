
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

fn qap_solve_fant(_a: &Matrix, _b: &Matrix, mut p: Vector, _r: i64, _m: i64) -> i64 {
    0
}

fn init_trace(_n: i64, _inc: i64, _trace: &mut Matrix) {
}

fn gen_trace(_p: &Vector, _trace: &Matrix) {
}

fn perm(_p: &mut Vector) {   
}

fn unif(_low: i64, _high: i64) -> i64 {
    0  
}

fn cost(_a: &Matrix, _b: &Matrix, _p: &Vector) -> i64 {
    0
}

fn local_search(_a: &Matrix, _b: &Matrix, _p: &mut Vector, _cost: &mut i64) {
}

fn delta(_a: &Matrix, _b: &Matrix, _p: &Vector, _r: i64, _s: i64) -> i64 {
    0
}

fn update_trace(_n: i64, _p: &Vector, _best_p: &Vector, _inc: &mut i64, _r: i64, _trace: &mut Matrix) {
}

