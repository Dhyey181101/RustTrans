

use std::boxed::Box;

fn cost(a: &Box<Matrix>, b: &Box<Matrix>, p: &Box<Vector>) -> i64 {
    let mut c = 0;
    let len = (*p).len();
    for i in 0..len {
        for j in 0..len {
            c += a.get(i, j) * b.get(j, i);
        }
    }
    c
}

struct Vector(Vec<i64>);

impl Vector {
    fn len(&self) -> i64 {
        (self.0).len() as i64
    }
}

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n) as usize + (j as usize)]
    }
}

