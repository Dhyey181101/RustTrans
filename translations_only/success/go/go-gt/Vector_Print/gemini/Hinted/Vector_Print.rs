
use std::fmt;

pub struct Vector(Vec<i64>);

impl Vector {
    pub fn print(&self) {
        for i in &self.0 {
            print!("{} ", i);
        }
        println!();
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
