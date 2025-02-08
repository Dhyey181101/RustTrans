
use std::cmp::Ordering;

struct Env {
    n: i64,
    t: Vec<bool>,
    s: Vec<bool>,
    slack: Vec<i64>,
    lx: Vec<i64>,
    ly: Vec<i64>,
}

fn min(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => a,
        _ => b,
    }
}

fn update(env: &mut Env) {
    let mut d = i64::max_value();
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
