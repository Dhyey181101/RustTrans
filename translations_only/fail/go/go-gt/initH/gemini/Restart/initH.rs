
use std::cmp::max;
use std::collections::HashMap;

fn init_h(g: &Matrix) -> Box<Env> {
    let mut e = Box::new(Env {
        m: 0,
        n: g.n,
        g: g.clone(),
        t: vec![false; g.n as usize],
        s: vec![false; g.n as usize],
        slack: vec![0; g.n as usize],
        slackx: vec![0; g.n as usize],
        prev: vec![0; g.n as usize],
        xy: vec![-1; g.n as usize],
        yx: vec![-1; g.n as usize],
        lx: vec![0; g.n as usize],
        ly: vec![0; g.n as usize],
    });
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = max(e.lx[i as usize], e.g.get(i, j));
        }
    }
    e
}

fn new_env(n: i64) -> Box<Env> {
    let mut e = Box::new(Env {
        m: 0,
        n: n,
        g: Matrix { n: 0, a: vec![] },
        t: vec![false; n as usize],
        s: vec![false; n as usize],
        slack: vec![0; n as usize],
        slackx: vec![0; n as usize],
        prev: vec![0; n as usize],
        xy: vec![-1; n as usize],
        yx: vec![-1; n as usize],
        lx: vec![0; n as usize],
        ly: vec![0; n as usize],
    });
    for i in 0..n {
        e.xy[i as usize] = -1;
        e.yx[i as usize] = -1;
    }
    e
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

#[derive(Clone)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Env {
    m: i64,
    n: i64,
    g: Matrix,
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
