

use std::i64;

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
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

impl Env {
    fn augment(&mut self) {
        let mut i: i64 = 0;
        let mut j: i64 = 0;
        let mut wr: i64 = 0;
        let mut rd: i64 = 0;
        let mut r: i64 = 0;
        let mut q: Vec<i64> = vec![0; self.n as usize];
        wr = 0;
        rd = 0;
        let mut q_index: usize = 0;
        if self.m == self.n {
            return;
        }
        for i in 0..self.n {
            if self.xy[i as usize] == -1 {
                wr += 1;
                q[q_index] = i;
                q_index += 1;
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
        let mut done: bool = false;
        loop {
            for rd in 0..wr {
                let current_index: usize = rd as usize;
                i = q[current_index];
                let mut j_index: usize = 0;
                for j in 0..self.n {
                    if self.g.get(i, j) == self.lx[i as usize] + self.ly[j as usize] && !self.t[j as usize] {
                        if self.yx[j as usize] == -1 {
                            break;
                        }
                        self.t[j as usize] = true;
                        wr += 1;
                        q_index += 1;
                        q.push(self.yx[j as usize]);
                        self.add(self.yx[j as usize], i);
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
            q_index = 0;
            for j in 0..self.n {
                if !self.t[j as usize] && self.slack[j as usize] == 0 {
                    if self.yx[j as usize] == -1 {
                        i = self.slackx[j as usize];
                        break;
                    } else {
                        self.t[j as usize] = true;
                        if !self.s[self.yx[j as usize] as usize] {
                            wr += 1;
                            q_index += 1;
                            q.push(self.yx[j as usize]);
                            self.add(self.yx[j as usize], self.slackx[j as usize]);
                        }
                    }
                }
            }
            if j < self.n {
                return;
            }
        }
        if j < self.n {
            self.m += 1;
            let mut current: i64 = -2;
            let mut next: i64 = 0;
            loop {
                next = self.xy[current as usize as usize];
                self.yx[next as usize] = current;
                self.xy[current as usize as usize] = next;
                current = self.prev[current as usize as usize];
                if current == -2 {
                    break;
                }
            }
            self.augment();
        }
    }

    fn add(&mut self, i: i64, p: i64) {
        let mut j: i64 = 0;
        self.s[i as usize] = true;
        self.prev[i as usize] = p;
        for j in 0..self.n {
            if self.lx[i as usize] + self.ly[i as usize] - self.g.get(i, j) < self.slack[i as usize] {
                self.slack[i as usize] = self.lx[i as usize] + self.ly[i as usize] - self.g.get(i, j);
                self.slackx[i as usize] = j;
            }
        }
    }

    fn update(&mut self) {
        let mut i: i64 = 0;
        let mut d: i64 = i64::MAX;
        for i in 0..self.n {
            if !self.t[i as usize] {
                d = std::cmp::min(d, self.slack[i as usize]);
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

fn min(a: i64, b: i64) -> i64 {
    if a < b {
        return a;
    }
    return b;
}

