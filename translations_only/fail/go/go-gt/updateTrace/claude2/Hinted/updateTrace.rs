
use std::ops::IndexMut;

type Vector = Vec<i64>;

fn update_trace(n: i64, p: &Vector, best_p: &Vector, inc: &mut i64, r: i64, trace: &mut Matrix) {
    let mut i = 0;
    while i < n && p[i as usize] == best_p[i as usize] {
        i += 1;
    }
    if i == n {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n {
            set(trace, i as usize, p[i as usize] as usize, get(trace, i as usize, p[i as usize] as usize) + *inc);
            set(trace, i as usize, best_p[i as usize] as usize, get(trace, i as usize, best_p[i as usize] as usize) + r);
        }
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            set(trace, i as usize, j as usize, inc);
        }
    }
}

fn set(m: &mut Matrix, i: usize, j: usize, v: i64) {
    m.data[i][j] = v;
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.data[i][j]
}

struct Matrix {
    n: usize,
    data: Vec<Vec<i64>>,
}

