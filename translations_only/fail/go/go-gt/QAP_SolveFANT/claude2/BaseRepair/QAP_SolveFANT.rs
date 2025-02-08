
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
    
    fn swap(&mut self, i: usize, j: usize) {
        self.0.swap(i, j);
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

fn qap_solve_fant(a: &Matrix, b: &Matrix, mut p: Vector, r: i64, m: i64) -> i64 {
    let mut inc = 1;
    let mut i = 0;
    let mut c = 0;
    let n = p.len();
    let mut w = Vector(vec![0; n as usize]);
    w.copy(&p);
    let mut trace = Matrix::new(n);
    init_trace(n, inc, &mut trace);
    let mut cc = INF;

    for _ in 0..m {
        gen_trace(&mut w, &trace);
        c = cost(a, b, &w);
        local_search(a, b, &mut w, &mut c);
        if c < cc {
            cc = c;
            p.copy(&w);
            inc = 1;
            init_trace(n, inc, &mut trace);
        } else {
            update_trace(n, &mut w, &p, &mut inc, r, &mut trace);
        }
        i += 1;
    }

    cc
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    let mut i = 0;

    while i < n {
        let mut j = 0;
        while j < n {
            trace.set(i, j, inc);
            j += 1;
        }
        i += 1;
    }
}

fn gen_trace(_p: &mut Vector, _trace: &Matrix) {
  
}

fn perm(_p: &mut Vector) {
  
}

fn unif(_low: i64, _high: i64) -> i64 {
   0
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    0
}

fn local_search(_a: &Matrix, _b: &Matrix, _p: &mut Vector, _cost: &mut i64) {
  
}

fn delta(_a: &Matrix, _b: &Matrix, _p: &Vector, _r: i64, _s: i64) -> i64 {
   0
}

fn update_trace(_n: i64, _p: &mut Vector, _best_p: &Vector, _inc: &mut i64, _r: i64, _trace: &mut Matrix) -> () {
  
}

