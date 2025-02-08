

use std::ops::IndexMut;

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * self.n + j]
    }
}

struct Vector(Vec<usize>);

fn vector_len(v: &Vector) -> usize {
    v.0.len()
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: usize, s: usize) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s], p.0[s]) - b.get(p.0[r], p.0[r]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s], p.0[r]) - b.get(p.0[r], p.0[s]));

    let mut i = 0;
    while i < vector_len(p) {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i], p.0[s]) - b.get(p.0[i], p.0[r]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s], p.0[i]) - b.get(p.0[r], p.0[i]));
        }
        i += 1;
    }

    d
}

