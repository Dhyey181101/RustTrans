
pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}

impl Matrix {
    pub fn print(&self) {
        for i in 0..self.n {
            for j in 0..self.n {
                print!("{} ", self.get(i, j));
            }
            println!();
        }
    }

    pub fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
