
use std::ops::Index;

fn delta_part(a: &Matrix, b: &Matrix, dist: &mut Matrix, p: &Vector, i: i64, j: i64, r: i64, s: i64) -> i64 {
    (dist.get(i, j) + (a.get(r, i) - a.get(r, j) + a.get(s, j) - a.get(s, i)) *
        (b.get(p[s as usize], p[i as usize]) - b.get(p[s as usize], p[j as usize]) + b.get(p[r as usize], p[j as usize]) - b.get(p[r as usize], p[i as usize])) +
        (a.get(i, r) - a.get(j, r) + a.get(j, s) - a.get(i, s)) *
            (b.get(p[i as usize], p[s as usize]) - b.get(p[j as usize], p[s as usize]) + b.get(p[j as usize], p[r as usize]) - b.get(p[i as usize], p[r as usize])))
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

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
