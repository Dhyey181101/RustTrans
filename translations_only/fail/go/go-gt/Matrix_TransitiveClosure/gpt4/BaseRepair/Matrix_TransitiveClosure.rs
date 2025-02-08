
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

fn transitive_closure(g: &mut Matrix, mut n: Option<&mut Matrix>) {
    let mut i: i64;
    let mut j: i64;
    let mut k: i64;
    for i in 0..g.n {
        for j in 0..g.n {
            for k in 0..g.n {
                if g.get(i, k) > 0 && g.get(k, j) > 0 {
                    if g.get(i, j) == 0 || g.get(i, k) + g.get(k, j) < g.get(i, j) {
                        g.set(i, j, g.get(i, k) + g.get(k, j));
                        if let Some(ref mut n) = n {
                            n.set(i, j, k + 1);
                        }
                    }
                }
            }
        }
    }
}
