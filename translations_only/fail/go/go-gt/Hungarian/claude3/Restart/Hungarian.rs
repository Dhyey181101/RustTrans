

use std::cmp::{max, min};
use std::i64;

fn hungarian(g: &Matrix) -> (Vec<i64>, Vec<i64>) {
    let mut e = init_h(g);
    e.augment();
    (e.xy, e.yx)
}

fn init_h(g: &Matrix) -> Env {
    let mut e = Env::new(g.n);
    e.g = Box::new(g.clone());
    e.n = g.n;
    for i in 0..e.n {
        for j in 0..e.n {
            e.lx[i as usize] = max(e.lx[i as usize], g.get(i, j));
        }
    }
    e
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

impl Env {
    fn new(n: i64) -> Env {
        let mut e = Env {
            m: 0,
            n,
            g: Box::new(Matrix::new(n)),
            t: vec![false; n as usize],
            s: vec![false; n as usize],
            slack: vec![0; n as usize],
            slackx: vec![0; n as usize],
            prev: vec![0; n as usize],
            xy: vec![-1; n as usize],
            yx: vec![-1; n as usize],
            lx: vec![0; n as usize],
            ly: vec![0; n as usize],
        };
        e
    }

    fn augment(&mut self) {
        let mut wr = 0;
        let mut rd = 0;
        let mut q = vec![0; self.n as usize];
        if self.m == self.n {
            return;
        }
        for i in 0..self.n {
            if self.xy[i as usize] == -1 {
                wr += 1;
                q[wr as usize] = i;
                let mut r = i;
                self.prev[i as usize] = -2;
                self.s[i as usize] = true;
                for j in 0..self.n {
                    self.slack[j as usize] = self.lx[i as usize] + self.ly[j as usize] - self.g.get(i, j);
                    self.slackx[j as usize] = j;
                }
                break;
            }
        }
        'outer: loop {
            while rd < wr {
                rd += 1;
                let i = q[rd as usize];
                for j in 0..self.n {
                    if self.g.get(i, j) == self.lx[i as usize] + self.ly[j as usize] && !self.t[j as usize] {
                        if self.yx[j as usize] == -1 {
                            break 'outer;
                        }
                        self.t[j as usize] = true;
                        wr += 1;
                        q[wr as usize] = self.yx[j as usize];
                        self.add(self.yx[j as usize], i);
                    }
                }
            }
            self.update();
            wr = 0;
            rd = 0;
            for j in 0..self.n {
                if !self.t[j as usize] && self.slack[j as usize] == 0 {
                    if self.yx[j as usize] == -1 {
                        let i = self.slackx[j as usize];
                        break 'outer;
                    } else {
                        self.t[j as usize] = true;
                        if !self.s[self.yx[j as usize] as usize] {
                            wr += 1;
                            q[wr as usize] = self.yx[j as usize];
                            self.add(self.yx[j as usize], self.slackx[j as usize]);
                        }
                    }
                }
            }
        }
        self.m += 1;
        let mut i = q[rd as usize];
        let mut j = self.slackx[i as usize];
        self.yx[j as usize] = i;
        self.xy[i as usize] = j;
        while self.prev[i as usize] != -2 {
            let k = self.xy[i as usize];
            self.yx[j as usize] = i;
            self.xy[i as usize] = j;
            i = self.prev[i as usize];
            j = k;
        }
        self.augment();
    }

    fn add(&mut self, i: i64, p: i64) {
        let mut j = 0;
        self.s[i as usize] = true;
        self.prev[i as usize] = p;
        for j in 0..self.n {
            if self.lx[i as usize] + self.ly[j as usize] - self.g.get(i, j) < self.slack[i as usize] {
                self.slack[i as usize] = self.lx[i as usize] + self.ly[j as usize] - self.g.get(i, j);
                self.slackx[i as usize] = j;
            }
        }
    }

    fn update(&mut self) {
        let mut d = i64::MAX;
        for i in 0..self.n {
            if !self.t[i as usize] {
                d = min(d, self.slack[i as usize]);
            }
        }
        for i in 0..self.n {
            if self.s[i as usize] {
                self.lx[i as usize] -= d;
            }
        }
        for i in 0..self.n {
            if self.t[i as usize] {
                self.ly[i as usize] += d;
            }
        }
        for i in 0..self.n {
            if !self.t[i as usize] {
                self.slack[i as usize] -= d;
            }
        }
    }
}

#[derive(Clone)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n,
            a: vec![0; (n * n) as usize],
        }
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

