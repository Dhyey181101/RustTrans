
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct Vector(Vec<i64>);

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

impl Vector {
    fn len(&self) -> i64 {
        self.0.len() as i64
    }
}

fn cost(a: &Matrix, b: &Matrix, p: Vector) -> i64 {
    let mut c = 0;
    for i in 0..p.len() {
        for j in 0..p.len() {
            c += a.get(i, j) * b.get(p.0[i as usize], p.0[j as usize]);
        }
    }
    c
}

fn main() {}
