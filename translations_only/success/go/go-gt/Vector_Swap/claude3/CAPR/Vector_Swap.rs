
pub struct Vector(Vec<i64>);

impl Vector {
    pub fn swap(&mut self, i: i64, j: i64) {
        let x = self.0[i as usize];
        self.0[i as usize] = self.0[j as usize];
        self.0[j as usize] = x;
    }
}
