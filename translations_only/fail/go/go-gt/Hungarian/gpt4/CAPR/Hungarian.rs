
use std::cmp::{max, min};

struct Matrix {
    n: i64,
    a: Vec<i64>,
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

fn hungarian(g: &Matrix) -> (Vec<i64>, Vec<i64>) {
    let mut e = init_h(g);
    augment(&mut e);
    (e.xy, e.yx)
}

fn init_h(g: &Matrix) -> Box<Env> {
    let mut e = new_env(g.n);
    e.g = Box::new(Matrix { n: g.n, a: g.a.clone() });
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = max(e.lx[i as usize], g.get(i, j));
        }
    }
    e
}

fn new_env(n: i64) -> Box<Env> {
    Box::new(Env {
        m: 0,
        n,
        g: Box::new(Matrix { n: 0, a: vec![] }),
        t: vec![false; n as usize],
        s: vec![false; n as usize],
        slack: vec![0; n as usize],
        slackx: vec![0; n as usize],
        prev: vec![-1; n as usize],
        xy: vec![-1; n as usize],
        yx: vec![-1; n as usize],
        lx: vec![0; n as usize],
        ly: vec![0; n as usize],
    })
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn augment(e: &mut Env) {
    let mut wr = 0;
    let mut rd = 0;
    let mut q = vec![0; e.n as usize];
    let mut r = 0;
    if e.m == e.n {
        return;
    }
    for i in 0..e.n {
        if e.xy[i as usize] == -1 {
            q[wr as usize] = i;
            r = i;
            e.prev[i as usize] = -2;
            e.s[i as usize] = true;
            break;
        }
    }
    loop {
        let mut j = 0;
        while rd < wr {
            rd += 1;
            let i = q[rd as usize];
            for j in 0..e.n {
                if e.g.get(i, j) == e.lx[i as usize] + e.ly[j as usize] && !e.t[j as usize] {
                    if e.yx[j as usize] == -1 {
                        break;
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
                    r = e.slackx[j as usize];
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
            break;
        }
    }
    if r < e.n {
        e.m += 1;
        let mut j = r;
        while j != -2 {
            let k = e.xy[j as usize];
            e.yx[j as usize] = j;
            e.xy[j as usize] = j;
            j = e.prev[j as usize];
            if j != -2 {
                e.yx[j as usize] = k;
            }
        }
        augment(e);
    }
}

fn add(e: &mut Env, i: i64, p: i64) {
    e.s[i as usize] = true;
    e.prev[i as usize] = p;
    for j in 0..e.n {
        if e.lx[i as usize] + e.ly[j as usize] - e.g.get(i, j) < e.slack[j as usize] {
            e.slack[j as usize] = e.lx[i as usize] + e.ly[j as usize] - e.g.get(i, j);
            e.slackx[j as usize] = i;
        }
    }
}

fn update(e: &mut Env) {
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
        if e.t[i as usize] {
            e.ly[i as usize] += d;
        }
        if !e.t[i as usize] {
            e.slack[i as usize] -= d;
        }
    }
}
