
use std::boxed::Box;

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

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn add(e: &mut Env, i: i64, p: i64) {
    let mut j: i64 = 0;
    e.s[i as usize] = true;
    e.prev[i as usize] = p;
    for j in 0..e.n {
        if e.lx[i as usize] + e.ly[i as usize] - get(&e.g, i, j) < e.slack[i as usize] {
            e.slack[i as usize] = e.lx[i as usize] + e.ly[i as usize] - get(&e.g, i, j);
            e.slackx[i as usize] = j;
        }
    }
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}
