

use std::ops::IndexMut;

struct Vector(Vec<usize>);

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

fn get(matrix: &Matrix, i: usize, j: usize) -> i64 {
    matrix.a[i * matrix.n + j]  
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

    let p_s = p.0[s];
    let p_i = p.0[i];
    let p_j = p.0[j];
    let p_r = p.0[r];
    
    dist.a[i * dist.n + j] +
        (get(a, r, i) - get(a, r, j) + get(a, s, j) - get(a, s, i)) *
        (get(b, p_s, p_i) - get(b, p_s, p_j) + get(b, p_r, p_j) - get(b, p_r, p_i)) + 
        (get(a, i, r) - get(a, j, r) + get(a, j, s) - get(a, i, s)) * 
        (get(b, p_i, p_s) - get(b, p_j, p_s) + get(b, p_j, p_r) - get(b, p_i, p_r))
}

