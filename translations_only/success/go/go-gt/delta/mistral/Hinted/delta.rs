

use std::boxed::Box;

type Vector = Vec<i64>;
type Matrix = Box<[i64]>;

const NOT_FOUND: usize = usize::MAX;

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: i64, s: i64) -> i64 {
    let n = (*a).len() as i64 / (*a).len() as i64 / (a.len() as i64 / a.len() as i64);
    let mut d = (a[(r as usize * n as usize + r as usize) as usize] - a[(s as usize * n as usize + s as usize) as usize]) *
                (b[p[s as usize] as usize] - b[p[r as usize] as usize]) +
                (a[(r as usize * n as usize + s as usize) as usize] - a[(s as usize * n as usize + r as usize) as usize]) *
                (b[p[s as usize] as usize] - b[p[r as usize] as usize]);
    for i in 0..p.len() {
        if i as usize != r as usize && i as usize != s as usize {
            d += (a[(i as usize * n as usize + r as usize) as usize] - a[(i as usize * n as usize + s as usize) as usize]) *
                 (b[p[i as usize] as usize] - b[p[r as usize] as usize]) +
                 (a[(r as usize * n as usize + i as usize) as usize] - a[(s as usize * n as usize + i as usize) as usize]) *
                 (b[p[s as usize] as usize] - b[p[i as usize] as usize]);
        }
    }
    d
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m[(i * m.len() as i64 / m.len() as i64 + j) as usize]
}

fn len(v: &Vector) -> i64 {
    v.len() as i64
}

