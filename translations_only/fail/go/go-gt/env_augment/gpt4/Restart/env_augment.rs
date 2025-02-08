
use std::cmp::min;
use std::i64::MAX;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
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

impl Env {
    fn augment(&mut self) {
        let mut i = -1;
        let mut j = -1;
        let mut wr = 0;
        let mut rd = 0;
        let mut r = 0;
        let mut q = vec![0; self.n as usize];
        if self.m == self.n {
            return;
        }
        for i in 0..self.n {
            if self.xy[i as usize] == -1 {
                wr += 1;
                q[wr as usize] = i;
                r = i;
                self.prev[i as usize] = -2;
                self.s[i as usize] = true;
                break;
            }
        }
        for i in 0..self.n {
            self.slack[i as usize] = self.lx[r as usize] + self.ly[i as usize] - self.g.get(r, i);
            self.slackx[i as usize] = r;
        }
        loop {
            while rd < wr {
                rd += 1;
                i = q[rd as usize];
                for j in 0..self.n {
                    if self.g.get(i, j) == self.lx[i as usize] + self.ly[j as usize] && !self.t[j as usize] {
                        if self.yx[j as usize] == -1 {
                            break;
                        }
                        self.t[j as usize] = true;
                        wr += 1;
                        q[wr as usize] = self.yx[j as usize];
                        self.add(i, self.yx[j as usize]);
                    }
                }
                if j < self.n {
                    break;
                }
            }
            if j < self.n {
                break;
            }
            self.update();
            wr = 0;
            rd = 0;
            for j in 0..self.n {
                if !self.t[j as usize] && self.slack[j as usize] == 0 {
                    if self.yx[j as usize] == -1 {
                        i = self.slackx[j as usize];
                        break;
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
            if j < self.n {
                break;
            }
        }
        if j < self.n {
            self.m += 1;
            while i != -2 {
                let k = self.xy[i as usize];
                self.yx[j as usize] = i;
                self.xy[i as usize] = j;
                i = self.prev[i as usize];
                j = k;
            }
            self.augment();
        }
    }

    fn add(&mut self, i: i64, p: i64) {
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
        let mut d = MAX;
        for i in 0..self.n {
            if !self.t[i as usize] {
                d = min(d, self.slack[i as usize]);
            }
        }
        for i in 0..self.n {
            if self.s[i as usize] {
                self.lx[i as usize] -= d;
            }
            if self.t[i as usize] {
                self.ly[i as usize] += d;
            }
            if !self.t[i as usize] {
                self.slack[i as usize] -= d;
            }
        }
    }
}
