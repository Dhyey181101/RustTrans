
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

fn delta(a: &Matrix, b: &Matrix, p: Vector, r: i64, s: i64) -> i64 {
    let mut d = (a.get(r, r) - a.get(s, s)) * (b.get(p.0[s as usize], p.0[s as usize]) - b.get(p.0[r as usize], p.0[r as usize]))
        + (a.get(r, s) - a.get(s, r)) * (b.get(p.0[s as usize], p.0[r as usize]) - b.get(p.0[r as usize], p.0[s as usize]));
    for i in 0..p.len() {
        if i != r && i != s {
            d += (a.get(i, r) - a.get(i, s)) * (b.get(p.0[i as usize], p.0[s as usize]) - b.get(p.0[i as usize], p.0[r as usize]))
                + (a.get(r, i) - a.get(s, i)) * (b.get(p.0[s as usize], p.0[i as usize]) - b.get(p.0[r as usize], p.0[i as usize]));
        }
    }
    d
}
