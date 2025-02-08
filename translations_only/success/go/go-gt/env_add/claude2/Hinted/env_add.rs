
use std::ops::IndexMut;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

struct Env {
    n: i64,
    g: Box<Matrix>,
    s: Vec<bool>,
    slack: Vec<i64>,
    slackx: Vec<i64>,
    prev: Vec<i64>,
    lx: Vec<i64>,
    ly: Vec<i64>,
}

fn add(e: &mut Env, i: i64, p: i64) {
    let mut j = 0;
    e.s[i as usize] = true;
    e.prev[i as usize] = p;
    while j < e.n {
        if e.lx[i as usize] + e.ly[i as usize] - e.g.get(i, j) < e.slack[i as usize] {
            e.slack[i as usize] = e.lx[i as usize] + e.ly[i as usize] - e.g.get(i, j);
            e.slackx[i as usize] = j;
        }
        j += 1;
    }
}
