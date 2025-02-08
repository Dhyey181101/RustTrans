
use std::ops::Index;

fn delta(a: &Box<Matrix>, b: &Box<Matrix>, p: Vector, r: i64, s: i64) -> i64 {
    let mut d = ((matrix_get(a, r, r) - matrix_get(a, s, s)) * (matrix_get(b, p[s as usize], p[s as usize]) - matrix_get(b, p[r as usize], p[r as usize])) +
                 (matrix_get(a, r, s) - matrix_get(a, s, r)) * (matrix_get(b, p[s as usize], p[r as usize]) - matrix_get(b, p[r as usize], p[s as usize]))) as i64;

    for i in 0..vector_len(&p) {
        if i != r && i != s {
            d += ((matrix_get(a, i, r) - matrix_get(a, i, s)) * (matrix_get(b, p[i as usize], p[s as usize]) - matrix_get(b, p[i as usize], p[r as usize])) +
                  (matrix_get(a, r, i) - matrix_get(a, s, i)) * (matrix_get(b, p[s as usize], p[i as usize]) - matrix_get(b, p[r as usize], p[i as usize]))) as i64;
        }
    }

    d
}

fn matrix_get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

fn vector_len(v: &Vector) -> i64 {
    v.len() as i64
}

type Vector = Vec<i64>;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Index<i64> for Matrix {
    type Output = [i64];

    fn index(&self, index: i64) -> &[i64] {
        let start = (index * self.n) as usize;
        let end = start + self.n as usize;
        &self.a[start..end]
    }
}
