
struct Vector(Vec<i64>);

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Matrix { n, a: vec![0; (n * n) as usize] }
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        let index = (i * self.n + j) as usize;
        self.a[index] = v;
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        let index = (i * self.n + j) as usize;
        self.a[index]
    }
}

fn update_trace(n: i64, p: &Vector, best_p: &Vector, inc: &mut i64, r: i64, trace: &mut Matrix) {
    let mut i = 0;
    while i < n && p.0[i as usize] == best_p.0[i as usize] {
        i += 1;
    }
    if i == n {
        *inc += 1;
        init_trace(n, *inc, trace);
    } else {
        for i in 0..n {
            trace.set(i, p.0[i as usize], trace.get(i, p.0[i as usize]) + *inc);
            trace.set(i, best_p.0[i as usize], trace.get(i, best_p.0[i as usize]) + r);
        }
    }
}

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    for i in 0..n {
        for j in 0..n {
            trace.set(i, j, inc);
        }
    }
}
