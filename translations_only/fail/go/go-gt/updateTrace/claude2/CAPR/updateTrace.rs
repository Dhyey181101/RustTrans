

use std::ops::IndexMut;

type Vector = Vec<i64>;

fn update_trace(n: usize, p: &Vector, best_p: &Vector, inc: &mut i64, r: i64, trace: &mut Box<Matrix>) {
    let mut i = 0;
    while i < n && p[i] == best_p[i] {
        i += 1;
    }
    if i == n {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n {
            set(trace, i as i64, p[i] as i64, get(trace, i as i64, p[i] as i64) + *inc);
            set(trace, i as i64, best_p[i] as i64, get(trace, i as i64, best_p[i] as i64) + r);
        }
    }
}

fn init_trace(n: usize, inc: i64, trace: &mut Box<Matrix>) {
    for i in 0..n {
        for j in 0..n {
            set(trace, i as i64, j as i64, inc);
        }
    }
}

fn set(m: &mut Box<Matrix>, i: i64, j: i64, v: i64) {
    m.a[i as usize][j as usize] = v;
}

fn get(m: &Box<Matrix>, i: i64, j: i64) -> i64 {
    m.a[i as usize][j as usize] 
}

struct Matrix {
    n: usize,
    a: Vec<Vec<i64>>,
}

