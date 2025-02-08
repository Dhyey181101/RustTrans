
use std::boxed::Box;

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while j < n {
            set(trace, i, j, inc);
            j += 1;
        }
        i += 1;
    }
}

fn set(m: &mut Matrix, i: i64, j: i64, v: i64) {
    if i >= 0 && i < m.n && j >= 0 && j < m.n {
        unsafe {
            *m.a.get_unchecked_mut((i * m.n + j) as usize) = v; 
        }
    } else {
        println!("Input is invalid, crash gracefully");
    }
}
