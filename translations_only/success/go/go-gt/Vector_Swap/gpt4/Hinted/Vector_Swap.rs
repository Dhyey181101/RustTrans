
struct Vector(Vec<i64>);

impl Vector {
    fn swap(&mut self, i: usize, j: usize) {
        let x = self.0[i];
        self.0[i] = self.0[j];
        self.0[j] = x;
    }
}

fn main() {}
