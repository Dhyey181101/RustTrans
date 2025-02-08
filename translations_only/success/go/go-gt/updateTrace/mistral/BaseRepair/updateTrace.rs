
use std::boxed::Box;

type Vector = Vec<i64>;
type Matrix = StructMatrix;

fn update_trace(n: i64, p: Vector, best_p: Vector, inc: &mut i64, r: i64, trace: &mut StructMatrix) {
    let mut i = 0;
    while i < n && p[i as usize] == best_p[i as usize] {
        i += 1;
    }
    if i == n {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n {
            trace.set(i, p[i as usize], trace.get(i, p[i as usize]) + *inc);
            trace.set(i, best_p[i as usize], trace.get(i, best_p[i as usize]) + r);
        }
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut StructMatrix) {
    let mut i = 0;
    let mut j = 0;
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}

struct StructMatrix {
    n: i64,
    a: Box<[i64]>,
}

impl StructMatrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
