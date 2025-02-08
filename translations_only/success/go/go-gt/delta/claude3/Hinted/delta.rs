
use std::ops::Index;

pub fn delta(a: &Box<Matrix>, b: &Box<Matrix>, p: Vector, r: usize, s: usize) -> i64 {
    let mut d = ((a[r][r] - a[s][s]) * (b[p[s]][p[s]] - b[p[r]][p[r]]) +
                 (a[r][s] - a[s][r]) * (b[p[s]][p[r]] - b[p[r]][p[s]])) as i64;

    for i in 0..p.len() {
        if i != r && i != s {
            d += ((a[i][r] - a[i][s]) * (b[p[i]][p[s]] - b[p[i]][p[r]]) +
                  (a[r][i] - a[s][i]) * (b[p[s]][p[i]] - b[p[r]][p[i]])) as i64;
        }
    }

    d
}

pub struct Matrix {
    n: usize,
    a: Vec<i64>,
}

impl Index<usize> for Matrix {
    type Output = [i64];

    fn index(&self, i: usize) -> &[i64] {
        let start = i * self.n;
        let end = start + self.n;
        &self.a[start..end]
    }
}

pub type Vector = Vec<usize>;
