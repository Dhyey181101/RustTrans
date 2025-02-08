

use std::boxed::Box;

type Vector = Vec<i64>;
type Matrix = Box<[i64]>;

const NOT_IMPLEMENTED: i64 = 0;

fn delta_part(a: &Matrix, b: &Matrix, dist: &Matrix, p: &Vector, i: usize, j: usize, r: usize, s: usize) -> i64 {
    let n = (*dist).len() as i64 / (*dist).len() as i64 / (*dist).len() as i64;
    let a_elem = |i: usize, j: usize| (*a)[(i * n as usize + j) as usize];
    let b_elem = |i: usize, j: usize| (*b)[(i * n as usize + j) as usize];
    let dist_elem = |i: usize, j: usize| (*dist)[(i * n as usize + j) as usize];
    
    dist_elem(i, j) + (a_elem(r, i) - a_elem(r, j) + a_elem(s, j) - a_elem(s, i)) *
    (b_elem(p[s] as usize, p[i] as usize) - b_elem(p[s] as usize, p[j] as usize) + b_elem(p[r] as usize, p[j] as usize) - b_elem(p[r] as usize, p[i] as usize)) +
    (a_elem(i, r) - a_elem(j, r) + a_elem(j, s) - a_elem(i, s)) *
    (b_elem(p[i] as usize, p[s] as usize) - b_elem(p[j] as usize, p[s] as usize) + b_elem(p[j] as usize, p[r] as usize) - b_elem(p[i] as usize, p[r] as usize))
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m[(i * m.len() as usize + j) as usize]
}

