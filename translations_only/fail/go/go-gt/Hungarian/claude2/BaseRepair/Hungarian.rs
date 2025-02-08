

use std::cmp::{max, min}; 

#[derive(Clone)] 
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[i as usize * m.n as usize + j as usize]  
}

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

fn new_env(n: i64) -> Env {
    Env {
        m: 0,
        n,
        g: Box::new(Matrix { n: 0, a: vec![] }),
        t: vec![false; n as usize],
        s: vec![false; n as usize],
        slack: vec![-1; n as usize],
        slackx: vec![-1; n as usize],
        prev: vec![-1; n as usize],
        xy: vec![-1; n as usize],
        yx: vec![-1; n as usize],
        lx: vec![-1; n as usize],
        ly: vec![-1; n as usize],
    }
}

fn init_h(g: &Matrix) -> Env {
    let mut e = new_env(g.n);
    e.g = Box::new(g.clone());
    e.n = g.n;

    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = max(e.lx[i as usize], get(g, i, j));
        }
    }

    e
}

fn hungarian(g: &Matrix) -> (Vec<i64>, Vec<i64>) {
    let mut e = init_h(&g);
    augment(&mut e);
    (e.xy, e.yx)  
}

fn augment(e: &mut Env) {
    let mut i: i64;
    let mut j: i64 = 0;
    let mut wr: i64 = 0;
    let mut rd: i64 = 0;
    let mut r: i64 = 0;
    let mut q = vec![-1; e.n as usize];

    if e.m == e.n {
        return;
    }

    'outer: for i in 0..e.n {
        if e.xy[i as usize] == -1 {
            wr += 1;
            q[wr as usize] = i;
            r = i;
            e.prev[i as usize] = -2;
            e.s[i as usize] = true;
            break;
        }
    }

    for i in 0..e.n {
        e.slack[i as usize] = e.lx[r as usize] + e.ly[i as usize] - get(e.g.as_ref(), r, i);
        e.slackx[i as usize] = r;
    }

    'inner: loop {
        for rd in 0..e.n {
            i = q[rd as usize];

            for j in 0..e.n {
                if get(e.g.as_ref(), i, j) == e.lx[i as usize] + e.ly[j as usize] && !e.t[j as usize] {
                    if e.yx[j as usize] == -1 {
                        break 'inner;
                    }

                    e.t[j as usize] = true;
                    wr += 1;
                    q[wr as usize] = e.yx[j as usize];
                    add(e, e.yx[j as usize], i);
                }
            }

            if j < e.n {
                break; 
            }
        }

        if j < e.n {
            break;
        }

        update(e);

        wr = 0;
        rd = 0;

        for j in 0..e.n {
            if !e.t[j as usize] && e.slack[j as usize] == 0 {
                if e.yx[j as usize] == -1 {
                    i = e.slackx[j as usize];
                    break;
                } else {
                    e.t[j as usize] = true;

                    if !e.s[e.yx[j as usize] as usize] {
                        wr += 1;
                        q[wr as usize] = e.yx[j as usize];
                        add(e, e.yx[j as usize], e.slackx[j as usize]);
                    }
                }
            }
        }

        if j < e.n {
            return;
        }
    }

    if j < e.n {
        e.m += 1;

        let mut i = -2;
        while i != -2 {
            let k = e.xy[i as usize];
            e.yx[j as usize] = i;
            e.xy[i as usize] = j;
            i = e.prev[i as usize];
            j = k;
        }

        augment(e);
    }
}

fn add(e: &mut Env, i: i64, p: i64) {
    e.s[i as usize] = true;
    e.prev[i as usize] = p;

    for j in 0..e.n {
        if e.lx[i as usize] + e.ly[i as usize] - get(e.g.as_ref(), i, j) < e.slack[i as usize] {
            e.slack[i as usize] = e.lx[i as usize] + e.ly[i as usize] - get(e.g.as_ref(), i, j);
            e.slackx[i as usize] = j;
        }
    }
}

fn update(e: &mut Env) {
    let mut i: i64;
    let mut d = i64::MAX;

    for i in 0..e.n {
        if !e.t[i as usize] {
            d = min(d, e.slack[i as usize]);
        }
    }

    for i in 0..e.n {
        if e.s[i as usize] {
            e.lx[i as usize] -= d;
        }
    }

    for i in 0..e.n {
        if e.t[i as usize] {
            e.ly[i as usize] += d;
        }
    }

    for i in 0..e.n {
        if !e.t[i as usize] {
            e.slack[i as usize] -= d;
        }
    }
}

