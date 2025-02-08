
use std::cmp::Ordering;

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

fn augment(e: &mut Env) {
    let mut i = 0;
    let mut j = 0;
    let mut wr = 0;
    let mut rd = 0;
    let mut r = 0;
    let mut q = vec![0; e.n as usize];

    if e.m == e.n {
        return;
    }

    for i in 0..e.n {
        if e.xy[i as usize] == -1 {
            wr = 1;
            q[wr as usize] = i;
            r = i;
            e.prev[i as usize] = -2;
            e.s[i as usize] = true;
            break;
        }
    }

    for i in 0..e.n {
        e.slack[i as usize] = e.lx[r as usize] + e.ly[i as usize] - e.g.get(r, i);
        e.slackx[i as usize] = r;
    }

    loop {
        for rd in 0..wr {
            i = q[rd as usize];
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
        let mut k = 0;
        loop {
            k = e.xy[i as usize];
            e.yx[j as usize] = i;
            e.xy[i as usize] = j;
            if i == -2 { break; }
            i = e.prev[i as usize];
            j = k;
        }
        augment(e);
    }
}

fn add(e: &mut Env, i: i64, p: i64) {
    let mut j = 0;
    e.s[i as usize] = true;
    e.prev[i as usize] = p;
    for j in 0..e.n {
        if e.lx[i as usize] + e.ly[i as usize] - e.g.get(i, j) < e.slack[i as usize] {
            e.slack[i as usize] = e.lx[i as usize] + e.ly[i as usize] - e.g.get(i, j);
            e.slackx[i as usize] = j; 
        }
    }
}

fn update(e: &mut Env) {
    let mut i = 0;
    let mut d = i64::MAX;
    for i in 0..e.n {
        if !e.t[i as usize] {
           d = d.min(e.slack[i as usize]); 
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

fn min(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => a,
        _ => b
    }
}
