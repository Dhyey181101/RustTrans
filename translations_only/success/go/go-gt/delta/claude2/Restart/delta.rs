
use std::convert::TryInto;

struct Matrix {
    n: i64,
    a: Vec<i64>, 
}

struct Vector(Vec<usize>);

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i * m.n as usize + j]
}

fn delta(a: &Matrix, b: &Matrix, p: &Vector, r: usize, s: usize) -> i64 {
    let mut d = (get(a, r, r) - get(a, s, s)) * (get(b, p.0[s].try_into().unwrap(), p.0[s].try_into().unwrap()) - get(b, p.0[r].try_into().unwrap(), p.0[r].try_into().unwrap()))
        + (get(a, r, s) - get(a, s, r)) * (get(b, p.0[s].try_into().unwrap(), p.0[r].try_into().unwrap()) - get(b, p.0[r].try_into().unwrap(), p.0[s].try_into().unwrap()));

    let len = p.0.len();
    let mut i = 0;
    while i < len {
        if i != r && i != s {
            let tmp = (get(a, i, r) - get(a, i, s)) * (get(b, p.0[i].try_into().unwrap(), p.0[s].try_into().unwrap()) - get(b, p.0[i].try_into().unwrap(), p.0[r].try_into().unwrap()))
                + (get(a, r, i) - get(a, s, i)) * (get(b, p.0[s].try_into().unwrap(), p.0[i].try_into().unwrap()) - get(b, p.0[r].try_into().unwrap(), p.0[i].try_into().unwrap()));
            d += tmp;
        }
        i += 1;
    }

    d
}

