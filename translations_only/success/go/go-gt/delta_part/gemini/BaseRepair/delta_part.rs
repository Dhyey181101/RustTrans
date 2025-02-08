
use std::ops::Index;
use std::ops::IndexMut;

fn delta_part(a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    return (dist[(i, j)] + (a[(r, i)] - a[(r, j)] + a[(s, j)] - a[(s, i)]) *
        (b[(p[s as usize], p[i as usize])] - b[(p[s as usize], p[j as usize])] + b[(p[r as usize], p[j as usize])] - b[(p[r as usize], p[i as usize])]) +
        (a[(i, r)] - a[(j, r)] + a[(j, s)] - a[(i, s)]) *
            (b[(p[i as usize], p[s as usize])] - b[(p[j as usize], p[s as usize])] + b[(p[j as usize], p[r as usize])] - b[(p[i as usize], p[r as usize])]))
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Index<(i64, i64)> for Matrix {
    type Output = i64;

    fn index(&self, (i, j): (i64, i64)) -> &Self::Output {
        &self.a[(i * self.n + j) as usize]
    }
}

impl IndexMut<(i64, i64)> for Matrix {
    fn index_mut(&mut self, (i, j): (i64, i64)) -> &mut Self::Output {
        &mut self.a[(i * self.n + j) as usize]
    }
}

type Vector = Vec<i64>;
