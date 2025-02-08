
use std::ops::Index;

fn delta(a: &Box<Matrix>, b: &Box<Matrix>, p: Vector, r: i64, s: i64) -> i64 {
    let mut d = ((a.a[r as usize * a.n as usize + r as usize] - a.a[s as usize * a.n as usize + s as usize]) * (b.a[p[s as usize] as usize] - b.a[p[r as usize] as usize]) +
                 (a.a[r as usize * a.n as usize + s as usize] - a.a[s as usize * a.n as usize + r as usize]) * (b.a[p[s as usize] as usize] - b.a[p[r as usize] as usize])) as i64;

    for i in 0..p.len() {
        if i != r as usize && i != s as usize {
            d += ((a.a[i * a.n as usize + r as usize] - a.a[i * a.n as usize + s as usize]) * (b.a[p[i] as usize] - b.a[p[s as usize] as usize]) +
                  (a.a[r as usize * a.n as usize + i] - a.a[s as usize * a.n as usize + i]) * (b.a[p[s as usize] as usize] - b.a[p[r as usize] as usize])) as i64;
        }
    }

    d
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Index<i64> for Matrix {
    type Output = [i64];

    fn index(&self, i: i64) -> &[i64] {
        let start = (i * self.n) as usize;
        let end = start + self.n as usize;
        &self.a[start..end]
    }
}

type Vector = Vec<i64>;
