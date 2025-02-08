
use std::cmp::max;
use std::collections::HashMap;

pub fn init_h(g: &Matrix) -> Box<Env> {
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

pub fn new_env(n: i64) -> Box<Env> {
    Box::new(Env {
        m: 0,
        n,
        g: Matrix { n, a: vec![] },
        t: vec![false; n as usize],
        s: vec![false; n as usize],
        slack: vec![0; n as usize],
        slackx: vec![0; n as usize],
        prev: vec![0; n as usize],
        xy: vec![-1; n as usize],
        yx: vec![-1; n as usize],
        lx: vec![0; n as usize],
        ly: vec![0; n as usize],
    })
}

impl Matrix {
    pub fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

#[derive(Clone)]
pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}

pub struct Env {
    pub m: i64,
    pub n: i64,
    pub g: Matrix,
    pub t: Vec<bool>,
    pub s: Vec<bool>,
    pub slack: Vec<i64>,
    pub slackx: Vec<i64>,
    pub prev: Vec<i64>,
    pub xy: Vec<i64>,
    pub yx: Vec<i64>,
    pub lx: Vec<i64>,
    pub ly: Vec<i64>,
}

