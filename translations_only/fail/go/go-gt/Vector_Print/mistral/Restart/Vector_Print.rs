

use std::fmt;

struct Vector(Box<[i64; 1000]>);

impl Vector {
    fn new() -> Vector {
        Vector(Box::new([0; 1000]))
    }

    fn set(&mut self, index: usize, value: i64) {
        self.0[index] = value;
    }

    fn print(&self) {
        for i in &self.0[..] {
            print!("{} ", i);
        }
        println!();
    }
}

fn main() {
    let mut v = Vector::new();
    v.set(0, 1);
    v.set(1, 2);
    v.set(2, 3);
    v.print();
}

