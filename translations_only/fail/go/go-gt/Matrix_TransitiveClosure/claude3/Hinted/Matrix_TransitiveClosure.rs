
use std::ops::Index;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

fn transitive_closure(g: &mut Matrix, n: &mut Option<Box<Matrix>>) {
    let mut i = 0;
    while i < g.n {
        let mut j = 0;
        while j < g.n {
            let mut k = 0;
            while k < g.n {
                if g.get(i, k) > 0 && g.get(k, j) > 0 {
                    if g.get(i, j) == 0 || g.get(i, k) + g.get(k, j) < g.get(i, j) {
                        g.set(i, j, g.get(i, k) + g.get(k, j));
                        if let Some(ref mut n) = *n {
                            n.set(i, j, k + 1);
                        }
                    }
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}
