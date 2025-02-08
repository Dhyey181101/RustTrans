

use std::mem;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[i as usize * self.n as usize + j as usize] = v;
    }
}

fn transitive_closure(g: &mut Matrix, n: &mut Matrix) {
    let g_n = g.n;
    for i in 0..g_n {
        for j in 0..g_n {
            for k in 0..g_n {
                if g.get(i, k) > 0 && g.get(k, j) > 0 {
                    if g.get(i, j) == 0 || g.get(i, k) + g.get(k, j) < g.get(i, j) {
                        g.set(i, j, g.get(i, k) + g.get(k, j));
                        n.set(i, j, k as i64 + 1);
                    }
                }
            }
        }
    }
}

