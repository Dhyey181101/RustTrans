
use std::cmp::{max, min}; 

#[derive(Clone)]
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

fn new_env(n: i64) -> Env {
    Env {
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

fn get(g: &Matrix, i: i64, j: i64) -> i64 {
    g.a[i as usize * g.n as usize + j as usize]
}

fn augment(env: &mut Env) {
    let mut wr: i64 = 0;
    let mut rd: i64 = 0;
    let mut r: i64 = 0;
    let mut q = vec![0; env.n as usize];
    let mut i = 0;
    let mut j = 0;

    if env.m == env.n {
        return;
    }

    for i in 0..env.n {
        if env.xy[i as usize] == -1 {
            wr += 1;
            q[wr as usize] = i;
            r = i;
            env.prev[i as usize] = -2;
            env.s[i as usize] = true;
            break; 
        }
    }

    for i in 0..env.n {
        env.slack[i as usize] = env.lx[r as usize] + env.ly[i as usize] - get(env.g.as_ref(), r, i);
        env.slackx[i as usize] = r; 
    }

    loop {
        loop {
            if rd >= wr {
                break;
            }

            rd += 1; 
            i = q[rd as usize];

            for j in 0..env.n {
                if get(env.g.as_ref(), i, j) == env.lx[i as usize] + env.ly[j as usize] && !env.t[j as usize] {
                    if env.yx[j as usize] == -1 {    
                        break;
                    }

                    env.t[j as usize] = true;    
                    wr += 1;
                    q[wr as usize] = env.yx[j as usize];
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
                    i = env.slackx[j as usize];
                    break;
                } else {
                    env.t[j as usize] = true;    
                    if !env.s[env.yx[j as usize] as usize] {
                        wr += 1;
                        q[wr as usize] = env.yx[j as usize];
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
        let p = env.xy[i as usize];
        env.yx[j as usize] = i;
        env.xy[i as usize] = j;

        let mut k = p;
        while i != -2 {
            let tmp = i;
            i = env.prev[i as usize];
            k = tmp; 
        }

        augment(env);
    }
}

fn add(env: &mut Env, i: i64, p: i64) {
    env.s[i as usize] = true;
    env.prev[i as usize] = p;

    for j in 0..env.n {
        if env.lx[i as usize] + env.ly[i as usize] - get(env.g.as_ref(), i, j) < env.slack[i as usize] {
            env.slack[i as usize] = env.lx[i as usize] + env.ly[i as usize] - get(env.g.as_ref(), i, j);
            env.slackx[i as usize] = j;
        }
    }
}  

fn update(env: &mut Env) {
    let mut d = i64::MAX;

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

