

use std::vec::Vec;

type Vector = Vec<i64>;

fn update_trace(n: i64, p: Vector, best_p: Vector, inc: &mut Box<i64>, r: i64, trace: &mut Box<Matrix>) {
    let mut i = 0;
    while i < n && p[i as usize] == best_p[i as usize] {
        i += 1;
    }
    if i == n {
        **inc += 1;
        init_trace(n, **inc, trace);
    } else {
        for i in 0..n {
            set(trace, i, p[i as usize], get(trace, i, p[i as usize]) + **inc);
            set(trace, i, best_p[i as usize], get(trace, i, best_p[i as usize]) + r);
        }
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Box<Matrix>) {
    for i in 0..n {
        for j in 0..n {
            set(trace, i, j, inc);
        }
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn set(matrix: &mut Box<Matrix>, i: i64, j: i64, v: i64) {
    let n = matrix.n;
    matrix.a[(i * n + j) as usize] = v;
}

fn get(matrix: &Box<Matrix>, i: i64, j: i64) -> i64 {
    let n = matrix.n;
    matrix.a[(i * n + j) as usize]
}

