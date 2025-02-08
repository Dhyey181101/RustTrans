
use std::cmp;

struct Env {
    n: i64,
    t: Box<[bool]>,
    s: Box<[bool]>,
    slack: Box<[i64]>,
    lx: Box<[i64]>,
    ly: Box<[i64]>,
}

fn update(env: &mut Env) {
    let mut d = std::i64::MAX;
    for i in 0..env.n {
        if !env.t[i as usize] {
            d = min(d, env.slack[i as usize]);
        }
    }
    for i in 0..env.n {
        if env.s[i as usize] {
            env.lx[i as usize] -= d;
        }
    }
    for i in 0..env.n {
        if env.t[i as usize] {
            env.ly[i as usize] += d;
        }
    }
    for i in 0..env.n {
        if !env.t[i as usize] {
            env.slack[i as usize] -= d;
        }
    }
}

fn min(a: i64, b: i64) -> i64 {
    cmp::min(a, b)
}
