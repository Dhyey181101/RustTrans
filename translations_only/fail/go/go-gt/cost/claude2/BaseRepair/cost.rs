
use std::ops::IndexMut;

struct Vector(Vec<i64>);

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    let len = len(p);
    for i in 0..len {
        for j in 0..len {
            c += get(a, i as usize, j as usize) * get(b, p.0[i] as usize, p.0[j] as usize);
        }
    }
    c
}

fn len(v: &Vector) -> usize {
    v.0.len()
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i * m.n + j]
}

fn index_mut(v: &mut Vector, index: usize) -> &mut i64 {
    &mut v.0[index]
}

