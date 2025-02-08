
use std::ops::Index;

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a[(r, r)] - a[(s, s)]) * (b[(p[s as usize], p[s as usize])] - b[(p[r as usize], p[r as usize])])
        + (a[(r, s)] - a[(s, r)]) * (b[(p[s as usize], p[r as usize])] - b[(p[r as usize], p[s as usize])]);
    for i in 0..p.len() {
        if i != r && i != s {
            d += (a[(i, r)] - a[(i, s)]) * (b[(p[i as usize], p[s as usize])] - b[(p[i as usize], p[r as usize])])
                + (a[(r, i)] - a[(s, i)]) * (b[(p[s as usize], p[i as usize])] - b[(p[r as usize], p[i as usize])]);
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

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
}

impl Index<usize> for Vector {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
