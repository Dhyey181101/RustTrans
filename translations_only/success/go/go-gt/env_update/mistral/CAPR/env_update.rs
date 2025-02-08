

use std::cmp::Ordering;

struct Env {
    n: i64,
    t: Vec<bool>,
    s: Vec<bool>,
    slack: Vec<i64>,
    lx: Vec<i64>,
    ly: Vec<i64>,
}

impl Env {
    fn new(n: i64) -> Self {
        Env {
            n,
            t: vec![false; n as usize],
            s: vec![false; n as usize],
            slack: vec![0; n as usize],
            lx: vec![0; n as usize],
            ly: vec![0; n as usize],
        }
    }

    fn update(&mut self) {
        let mut i = 0;
        let mut d = i64::MAX;
        while i < self.n {
            if !self.t[i as usize] {
                d = std::cmp::min(d, self.slack[i as usize]);
            }
            i += 1;
        }
        i = 0;
        while i < self.n {
            if self.s[i as usize] {
                self.lx[i as usize] -= d;
            }
            i += 1;
        }
        i = 0;
        while i < self.n {
            if self.t[i as usize] {
                self.ly[i as usize] += d;
            }
            i += 1;
        }
        i = 0;
        while i < self.n {
            if !self.t[i as usize] {
                self.slack[i as usize] -= d;
            }
            i += 1;
        }
    }
}

fn min(a: i64, b: i64) -> i64 {
    a.min(b)
}

