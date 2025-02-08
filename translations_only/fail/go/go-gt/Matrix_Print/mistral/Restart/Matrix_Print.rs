
use std::fmt;

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * (self.n as usize) + j as usize]
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i = 0;
        let mut j = 0;
        for _ in 0..self.n {
            for _ in 0..self.n {
                write!(f, "{} ", self.get(i, j))?;
                j += 1;
            }
            i += 1;
            j = 0;
            writeln!(f)?;
        }
        Ok(())
    }
}
