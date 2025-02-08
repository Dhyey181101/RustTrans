

use std::cmp::min;

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

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * self.n as usize + j]
    }
}

fn augment(env: &mut Env) {
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut wr: i64 = 0;
    let mut rd: i64 = 0;
    let mut r: i64 = 0;
    let mut q: Vec<i64> = vec![0; env.n as usize];
    wr = 0;
    rd = 0;
    let mut q = vec![0; env.n as usize];
    if env.m == env.n {
        return;
    }
    for i in 0..env.n {
        if env.xy[i as usize] == -1 {
            wr += 1;
            q[wr as usize] = i as i64;
            r = i as i64;
            env.prev[i as usize] = -2;
            env.s[i as usize] = true;
            break;
        }
    }
    for i in 0..env.n {
        env.slack[i as usize] = env.lx[r as usize] + env.ly[i as usize] - env.g.get(r as usize, i as usize);
        env.slackx[i as usize] = r;
    }
    let mut j = 0;
    loop {
        let mut min_slack = i64::MAX;
        for k in 0..env.n {
            if env.s[k as usize] && env.slack[k as usize] < min_slack {
                min_slack = env.slack[k as usize];
                j = k as i64;
            }
        }
        if min_slack > 0 {
            break;
        }
        env.s[j as usize] = false;
        r = env.slackx[j as usize];
        env.prev[j as usize] = r;
        for k in 0..env.n {
            let new_val = env.ly[k as usize] - env.slack[j as usize] + env.g.get(r as usize, k as usize);
            if env.xy[k as usize] == r && new_val < env.lx[k as usize] {
                env.lx[k as usize] = new_val;
                env.yx[k as usize] = j as i64;
            }
        }
    }
    i = env.prev[j as usize];
    env.xy[i as usize] = j as i64;
    env.yx[j as usize] = i as i64;
    env.ly[j as usize] = env.slack[j as usize];
    env.slack[j as usize] = -1;
    while r != i {
        let new_val = env.lx[i as usize] + env.g.get(i as usize, r as usize) - env.ly[r as usize];
        if new_val < env.slack[r as usize] {
            env.slack[r as usize] = new_val;
            env.prev[r as usize] = i;
        }
        let tmp = i;
        i = r;
        r = tmp;
    }
}

