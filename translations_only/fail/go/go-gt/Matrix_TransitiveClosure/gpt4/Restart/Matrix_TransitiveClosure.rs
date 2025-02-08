
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn new(n: i64) -> Self {
        Self { n, a: vec![0; (n * n) as usize] }
    }

    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    fn set(&mut self, i: i64, j: i64, v: i64) {
        self.a[(i * self.n + j) as usize] = v;
    }
}

fn transitive_closure(g: &mut Matrix, mut n: Option<&mut Matrix>) {
    let size = g.n;
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
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

fn main() {
    let mut g = Matrix::new(4);
    let mut n = Matrix::new(4);
    transitive_closure(&mut g, Some(&mut n));
}
