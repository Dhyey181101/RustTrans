
use std::cmp::{max, min}; 

#[derive(Clone)]
struct Matrix {
    n: i64,
    a: Vec<i64>, 
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[i as usize]
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

fn new_env(n: i64) -> Env {
    Env {
        m: 0,
        n,
        g: Box::new(Matrix { n: 0, a: vec![] }),
        t: vec![false; n as usize],
        s: vec![false; n as usize],
        slack: vec![0; n as usize],
        slackx: vec![0; n as usize],
        prev: vec![-1; n as usize],
        xy: vec![-1; n as usize],
        yx: vec![-1; n as usize],
        lx: vec![0; n as usize],
        ly: vec![0; n as usize],
    }
}

fn init_h(g: &Matrix) -> Env {
    let mut e = new_env(g.n);
    e.g = Box::new(g.clone());
    e.n = g.n;
    
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = max(e.lx[i as usize], get(&g, i, j));  
        }
    }

    e   
}

fn hungarian(g: &Matrix) -> (Vec<i64>, Vec<i64>) {
    let mut e = init_h(&g);
    augment(&mut e);
    (e.xy, e.yx)  
}

fn augment(e: &mut Env) {
    // Function body
}
