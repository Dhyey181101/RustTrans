
struct Env {
    n: i64,
    g: Box<Matrix>,
    s: Vec<bool>,
    slack: Vec<i64>,
    slackx: Vec<i64>,
    prev: Vec<i64>,
    lx: Vec<i64>,
    ly: Vec<i64>,
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Env {
    fn add(&mut self, i: i64, p: i64) {
        let mut j: i64;
        self.s[i as usize] = true;
        self.prev[i as usize] = p;
        for j in 0..self.n {
            let value = self.lx[i as usize] + self.ly[j as usize] - get(&self.g, i, j);
            if value < self.slack[i as usize] {
                self.slack[i as usize] = value;
                self.slackx[i as usize] = j;
            }
        }
    }
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}
