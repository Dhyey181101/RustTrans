
use std::boxed::Box;

type Vector = Vec<i64>;
type Matrix = Box<[(i64, i64)]>;

fn cost(a: &Matrix, b: &Matrix, p: &Vector) -> i64 {
    let mut c = 0;
    let len = p.len() as i64;
    for i in 0..len {
        for j in 0..len {
            c += a[(i * len + j) as usize].0 * b[p[i as usize] as usize].1 * b[p[j as usize] as usize].1;
        }
    }
    c
}

fn len(v: &Vector) -> i64 {
    v.len() as i64
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m[(i * m.len() as i64 + j) as usize].0
}
