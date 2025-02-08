
use std::cmp;

#[derive(Clone)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Env {
    m: i64,
    n: i64,
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

fn init_h(g: &Matrix) -> Box<Env> {
    let mut e = Box::new(new_env(g.n));
    e.g = Box::new(clone_matrix(g));
    e.n = g.n;
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = cmp::max(e.lx[i as usize], get(&e.g, i, j));
        }
    }
    e
}

fn new_env(n: i64) -> Env {
    Env {
        m: 0,
        n,
        g: Box::new(Matrix{n: 0, a: vec![]}),
        t: vec![false; n as usize],
        s: vec![false; n as usize],
        slack: vec![0; n as usize],
        slackx: vec![0; n as usize],
        prev: vec![0; n as usize],
        xy: vec![-1; n as usize],
        yx: vec![-1; n as usize],
        lx: vec![0; n as usize],
        ly: vec![0; n as usize],
    }
}

fn clone_matrix(m: &Matrix) -> Matrix {
    Matrix {
        n: m.n,
        a: m.a.clone(),
    }
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[i as usize * m.n as usize + j as usize] 
}

fn max(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

