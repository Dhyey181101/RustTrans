
fn update_trace(n: i64, p: &Vec<i64>, best_p: &Vec<i64>, inc: &mut i64, r: i64, trace: &mut Matrix) {
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

fn init_trace(n: i64, inc: i64, trace: &mut Matrix) {
    let mut i = 0;
    let mut j = 0;
    while i < n {
        while j < n {
            trace.set(i, j, inc);
            j += 1;
        }
        j = 0;
        i += 1;
    }
}

impl Matrix {
    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}
