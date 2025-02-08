

use std::boxed::Box;

type Vector = Vec<i64>;
type Matrix = Box<MatrixImpl>;

struct MatrixImpl {
    n: i64,
    a: Vec<i64>,
}

impl MatrixImpl {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }

    fn n(&self) -> i64 {
        self.n
    }
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p[s as usize] as i64, p[s as usize] as i64) - b.get(p[r as usize] as i64, p[r as usize] as i64))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p[s as usize] as i64, p[r as usize] as i64) - b.get(p[r as usize] as i64, p[s as usize] as i64));
    for i in 0..p.len() {
        if i != r as usize && i != s as usize {
            d += (a.get(i as i64, r) - a.get(i as i64, s)) * (b.get(p[i as usize] as i64, p[s as usize] as i64) - b.get(p[i as usize] as i64, p[r as usize] as i64))
                + (a.get(r, i as i64) - a.get(s, i as i64)) * (b.get(p[s as usize] as i64, p[i as usize] as i64) - b.get(p[r as usize] as i64, p[i as usize] as i64));
        }
    }
    d
}

fn main() {}

