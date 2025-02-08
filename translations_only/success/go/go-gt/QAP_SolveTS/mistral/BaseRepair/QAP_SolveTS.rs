
use rand::Rng;
use rand::thread_rng;

pub struct Vector([i64; 1000]);

pub struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn new(n: i64) -> Matrix {
        Matrix {
            n,
            a: vec![0; (n * n) as usize].into_boxed_slice(),
        }
    }

    pub fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    pub fn set(&mut self, i: i64, j: i64, val: i64) {
        self.a[(i * self.n + j) as usize] = val;
    }
}
