
use std::mem;

fn update(e: &mut Env) {
    let mut i = 0;
    let mut d = i64::max_value();
    for _ in 0..e.n {
        if !e.t[i] {
            d = d.min(e.slack[i]);
        }
        i += 1;
    }
    i = 0;
    for _ in 0..e.n {
        if e.s[i] {
            e.lx[i] = e.lx[i].saturating_sub(d);
        }
        i += 1;
    }
    i = 0;
    for _ in 0..e.n {
        if e.t[i] {
            e.ly[i] = e.ly[i].saturating_add(d);
        }
        i += 1;
    }
    i = 0;
    for _ in 0..e.n {
        if !e.t[i] {
            e.slack[i] = e.slack[i].saturating_sub(d);
        }
        i += 1;
    }
}

fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}

#[derive(Default)]
struct Env {
    n: i64,

    t: Vec<bool>,
    s: Vec<bool>,
    slack: Vec<i64>,

    lx: Vec<i64>,
    ly: Vec<i64>,
}
