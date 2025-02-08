
use std::ops::{Index, IndexMut};

fn delta_part(a: &Matrix, b: &Vector, dist: &mut Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    (dist[(i, j)] + (a[(r, i)] - a[(r, j)] + a[(s, j)] - a[(s, i)]) *
        (b[p[s as usize] as usize] - b[p[j as usize] as usize] + b[p[r as usize] as usize] - b[p[i as usize] as usize]) +
        (a[(i, r)] - a[(j, r)] + a[(j, s)] - a[(i, s)]) *
            (b[p[i as usize] as usize] - b[p[j as usize] as usize] + b[p[j as usize] as usize] - b[p[i as usize] as usize]))
}

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Index<(i64, i64)> for Matrix {
    type Output = i64;

    fn index(&self, (i, j): (i64, i64)) -> &i64 {
        &self.a[(i * self.n + j) as usize]
    }
}

impl IndexMut<(i64, i64)> for Matrix {
    fn index_mut(&mut self, (i, j): (i64, i64)) -> &mut i64 {
        &mut self.a[(i * self.n + j) as usize]
    }
}

type Vector = Box<[i64]>;
