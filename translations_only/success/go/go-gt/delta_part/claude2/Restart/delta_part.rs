
use std::ops::IndexMut;

struct Vector(Vec<usize>);

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * self.n + j]
    }
}

fn delta_part(
    a: &Matrix,
    b: &Matrix,
    dist: &Matrix,
    p: &Vector,
    i: usize,
    j: usize,
    r: usize,
    s: usize) -> i64 {

    let ip = p.0[i];
    let jp = p.0[j];
    let rp = p.0[r];
    let sp = p.0[s];

    dist.get(i, j) +
    (a.get(r, i) - a.get(r, j) + a.get(s, j) - a.get(s, i)) *
    (b.get(sp, ip) - b.get(sp, jp) + b.get(rp, jp) - b.get(rp, ip)) +
    (a.get(i, r) - a.get(j, r) + a.get(j, s) - a.get(i, s)) *
    (b.get(ip, sp) - b.get(jp, sp) + b.get(jp, rp) - b.get(ip, rp))
}

