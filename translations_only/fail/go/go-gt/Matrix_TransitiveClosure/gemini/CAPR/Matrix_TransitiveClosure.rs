
pub fn transitive_closure(g: &mut Matrix, n: &mut Matrix) {
    for i in 0..g.n {
        for j in 0..g.n {
            for k in 0..g.n {
                if g.get(i, k) > 0 && g.get(k, j) > 0 {
                    if g.get(i, j) == 0 || g.get(i, k) + g.get(k, j) < g.get(i, j) {
                        g.set(i, j, g.get(i, k) + g.get(k, j));
                        if !n.empty() {
                            n.set(i, j, k + 1);
                        }
                    }
                }
            }
        }
    }
}

impl Matrix {
    pub fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }

    pub fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[i as usize * self.n as usize + j as usize] = v;
    }

    pub fn empty(&self) -> bool {
        self.n == 0
    }
}

pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}
