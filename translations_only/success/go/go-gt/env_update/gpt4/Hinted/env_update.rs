
struct Env {
    n: i64,
    t: Vec<bool>,
    s: Vec<bool>,
    slack: Vec<i64>,
    lx: Vec<i64>,
    ly: Vec<i64>,
}

impl Env {
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

fn min(a: i64, b: i64) -> i64 {
    if a < b { a } else { b }
}
