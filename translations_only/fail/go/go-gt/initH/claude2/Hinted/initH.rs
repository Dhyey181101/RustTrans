
use std::cmp;

#[derive(Clone)]
struct Matrix {
    n: usize,
    a: Vec<i64>,
}

struct Env {
    m: usize,
    n: usize,
    g: Box<Matrix>,
    t: Vec<bool>,
    s: Vec<bool>,
    slack: Vec<i64>,
    slackx: Vec<i64>,
    prev: Vec<i64>,
    xy: Vec<i64>,
    yx: Vec<i64>,
    lx: Vec<i64>,
    ly: Vec<i64>,
}

fn get(matrix: &Matrix, i: usize, j: usize) -> i64 {
    matrix.a[i * matrix.n + j]
}

fn init_h(g: &Matrix) -> Box<Env> {
    let mut e = Box::new(new_env(g.n));
    e.g = Box::new(g.clone());
    e.n = g.n;
    
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i] = cmp::max(e.lx[i], get(&e.g, i, j));
        }
    }
    
    e
}

fn new_env(n: usize) -> Env {
    Env {
        m: 0,
        n,
        g: Box::new(Matrix{n: 0, a: vec![]}),
        t: vec![false; n],
        s: vec![false; n],
        slack: vec![0; n],
        slackx: vec![0; n],
        prev: vec![0; n],
        xy: vec![-1; n],
        yx: vec![-1; n],
        lx: vec![0; n],
        ly: vec![0; n],
    }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

