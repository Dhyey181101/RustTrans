
use std::boxed::Box;

struct Vector(Box<[i64]>);

impl Vector {
    fn new(values: &[i64]) -> Self {
        Vector(values.into())
    }

    fn get(&self, index: usize) -> &i64 {
        &self.0[index]
    }

    fn set(&mut self, index: usize, value: i64) {
        self.0[index] = value;
    }
}

fn swap(vector: &mut Vector, i: usize, j: usize) {
    let temp = vector.0[i];
    vector.0[i] = vector.0[j];
    vector.0[j] = temp;
}
