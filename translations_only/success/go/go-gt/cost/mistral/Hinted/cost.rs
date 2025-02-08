

use std::boxed::Box;

type Vector = Vec<i64>;
type Matrix = Box<[(i64, i64)]>;

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    let len = p.len() as i64;
    for i in 0..len {
        for j in 0..len {
            let index_a = (i * len + j) as usize;
            let p_i = p[i as usize] as usize;
            let p_j = p[j as usize] as usize;
            let index_b = p_i * len as usize + p_j;
            c += a[index_a].0 * b[index_b].0;
        }
    }
    c
}

fn len(v: &Vector) -> usize {
    v.len()
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m[(i * m.len() as i64 + j) as usize].0
}

