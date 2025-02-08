

use std::mem;

#[derive(Clone)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Env {
    m: i64,
    n: i64,
    g: Option<Box<Matrix>>,
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

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

fn max(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn init_h(g: &Matrix) -> Box<Env> {
    let mut e = Box::new(Env::new(g.n));
    let mut i = 0;
    let mut j = 0;
    e.g = Some(Box::new(g.clone()));
    e.n = g.n;
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = max(e.lx[i as usize], e.g.as_ref().unwrap().get(i, j));
        }
    }
    e
}

impl Env {
    fn new(n: i64) -> Env {
        Env {
            m: 0,
            n: n,
            g: None,
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
}

