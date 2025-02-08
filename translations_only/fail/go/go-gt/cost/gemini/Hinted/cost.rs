
fn cost(a: &Matrix, b: &Matrix, p: &[i64]) -> i64 {
    let mut c = 0;
    for i in 0..p.len() {
        for j in 0..p.len() {
            c += a.get(i as usize, j as usize) * b.get(p[i] as usize, p[j] as usize);
        }
    }
    c
}

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> usize {
        self.0.len()
    }
}

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * self.n + j]
    }
}
