
use std::ops::Index;

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a[(r, r)] - a[(s, s)]) * (b[(p.get(s), p.get(s))] - b[(p.get(r), p.get(r))])
        + (a[(r, s)] - a[(s, r)]) * (b[(p.get(s), p.get(r))] - b[(p.get(r), p.get(s))]);
    for i in 0..p.len() {
        if i != r && i != s {
            d += (a[(i, r)] - a[(i, s)]) * (b[(p.get(i), p.get(s))] - b[(p.get(i), p.get(r))])
                + (a[(r, i)] - a[(s, i)]) * (b[(p.get(s), p.get(i))] - b[(p.get(r), p.get(i))]);
        }
    }
    d
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Index<(i64, i64)> for Matrix {
    type Output = i64;

    fn index(&self, (i, j): (i64, i64)) -> &Self::Output {
        &self.a[i as usize * self.n as usize + j as usize]
    }
}

struct Vector {
    a: Vec<i64>,
}

impl Vector {
    fn len(&self) -> i64 {
        self.a.len() as i64
    }

    fn get(&self, i: i64) -> i64 {
        self.a[i as usize]
    }
}
