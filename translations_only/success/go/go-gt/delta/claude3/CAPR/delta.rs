
use std::ops::Index;

fn delta(a: &Box<Matrix>, b: &Box<Matrix>, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = ((get(a, r, r) - get(a, s, s)) * (get(b, p[s as usize], p[s as usize]) - get(b, p[r as usize], p[r as usize])) +
                 (get(a, r, s) - get(a, s, r)) * (get(b, p[s as usize], p[r as usize]) - get(b, p[r as usize], p[s as usize]))) as i64;

    for i in 0..len(p) {
        if i != r && i != s {
            d += ((get(a, i, r) - get(a, i, s)) * (get(b, p[i as usize], p[s as usize]) - get(b, p[i as usize], p[r as usize])) +
                  (get(a, r, i) - get(a, s, i)) * (get(b, p[s as usize], p[i as usize]) - get(b, p[r as usize], p[i as usize]))) as i64;
        }
    }

    d
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

fn len(v: &Vector) -> i64 {
    v.0.len() as i64
}

struct Vector(Vec<i64>);

impl Index<usize> for Vector {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Index<i64> for Matrix {
    type Output = [i64];

    fn index(&self, index: i64) -> &Self::Output {
        let start = (index * self.n) as usize;
        let end = start + self.n as usize;
        &self.a[start..end]
    }
}
