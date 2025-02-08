
use std::ops::IndexMut;

type Vector = Vec<i64>;

fn update_trace(n: i64, p: &Vector, best_p: &Vector, inc: &mut i64, r: i64, trace: &mut Box<Matrix>) {
    let mut i = 0_usize;
    while i < n as usize && p[i] == best_p[i] {
        i += 1;
    }
    if i == n as usize {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n as usize {
            set(trace, i, p[i] as usize, get(&*trace, i, p[i] as usize) + *inc);
            set(trace, i, best_p[i] as usize, get(&*trace, i, best_p[i] as usize) + r);
        }
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Box<Matrix>) {
    for i in 0..n as usize {
        for j in 0..n as usize {
            set(trace, i, j, inc);
        }
    }
}

fn set(m: &mut Box<Matrix>, i: usize, j: usize, v: i64) {
    m.a[i][j] = v;
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i][j]
}

struct Matrix {
    n: usize,
    a: Vec<Vec<i64>>,
}

