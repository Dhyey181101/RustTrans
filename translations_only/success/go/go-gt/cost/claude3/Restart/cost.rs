
use std::ops::Deref;

fn cost(a: &Box<Matrix>, b: &Box<Matrix>, p: &Vector) -> i64 {
    let mut c: i64 = 0;
    for i in 0..p.len() as i64 {
        for j in 0..p.len() as i64 {
            c += a.get(i, j) * b.get(p[i as usize], p[j as usize]);
        }
    }
    c
}

fn len(v: &Vector) -> i64 {
    v.len() as i64
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

struct Vector(Vec<i64>);

impl Deref for Vector {
    type Target = Vec<i64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        get(self, i, j)
    }
}
