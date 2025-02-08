

use std::cmp::min;

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

fn augment(env: &mut Env) {
    let mut i: i64 = 0;
    let mut j: i64 = 0;
    let mut wr: i64 = 0;
    let mut rd: i64 = 0;
    let mut r: i64 = -1;
    let mut q: Vec<i64> = Vec::with_capacity(env.n as usize);
    if env.m == env.n {
        return;
    }
    for i in 0..env.n {
        if env.xy[i as usize] == -1 {
            wr += 1;
            q.push(i);
            r = i;
            env.prev[i as usize] = -2;
            env.s[i as usize] = true;
            break;
        }
    }
    for i in 0..env.n {
        env.slack[i as usize] = env.lx[r as usize] + env.ly[i as usize] - get(&*env.g, r, i);
        env.slackx[i as usize] = r;
    }
    loop {
        while rd < wr {
            rd += 1;
            i = q[rd as usize];
            for j in 0..env.n {
                if get(&*env.g, i, j) == env.lx[i as usize] + env.ly[j as usize] && !env.t[j as usize] {
                    if env.yx[j as usize] == -1 {
                        break;
                    }
                    env.t[j as usize] = true;
                    wr += 1;
                    q.push(env.yx[j as usize]);
                    add(env, env.yx[j as usize], i);
                }
            }
            if j < env.n {
                break;
            }
        }
        if j < env.n {
            break;
        }
        update(env);
        wr = 0;
        rd = 0;
        for j in 0..env.n {
            if !env.t[j as usize] && env.slack[j as usize] == 0 {
                if env.yx[j as usize] == -1 {
                    r = env.slackx[j as usize];
                    break;
                } else {
                    env.t[j as usize] = true;
                    if !env.s[env.yx[j as usize] as usize] {
                        wr += 1;
                        q.push(env.yx[j as usize]);
                        add(env, env.yx[j as usize], env.slackx[j as usize]);
                    }
                }
            }
        }
        if j < env.n {
            return;
        }
    }
    if j < env.n {
        env.m += 1;
        let mut k: i64;
        while i != -2 {
            k = env.xy[i as usize];
            env.yx[k as usize] = i;
            env.xy[i as usize] = k;
            i = env.prev[i as usize];
            j = k;
        }
        augment(env);
    }
}

fn add(env: &mut Env, i: i64, p: i64) {
    let mut j: i64;
    env.s[i as usize] = true;
    env.prev[i as usize] = p;
    for j in 0..env.n {
        if env.lx[i as usize] + env.ly[j as usize] - get(&*env.g, i, j) < env.slack[i as usize] {
            env.slack[i as usize] = env.lx[i as usize] + env.ly[j as usize] - get(&*env.g, i, j);
            env.slackx[i as usize] = j;
        }
    }
}

fn update(env: &mut Env) {
    let mut d: i64 = std::i64::MAX;
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

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn get(matrix: &Matrix, i: i64, j: i64) -> i64 {
    matrix.a[(i * matrix.n + j) as usize]
}

