
use std::boxed::Box;

type Vector = Box<[i64]>;
type Matrix = Box<[i64]>;

fn update_trace(n: i64, p: Vector, best_p: Vector, inc: &mut i64, r: i64, trace: &mut Matrix) {
    let mut i = 0;
    while i < n && p[i as usize] == best_p[i as usize] {
        i += 1;
    }
    if i == n {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n {
            let p_i = p[i as usize];
            let best_p_i = best_p[i as usize];
            trace[(i * n + p_i as i64) as usize] += *inc;
            trace[(i * n + best_p_i as i64) as usize] += r;
        }
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    let len = (n * n) as usize;
    for i in 0..len {
        trace[i] = inc;
    }
}

fn get(m: &Matrix, i: i64, j: i64, n: i64) -> i64 {
    m[(i * n + j) as usize]
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64, n: i64) {
    m[(i * n + j) as usize] = v;
}
