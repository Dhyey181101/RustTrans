
use std::fmt;

#[derive(Debug)]
struct OrderedMap(Vec<(String, f32)>);

impl OrderedMap {
    fn new() -> Self {
        OrderedMap(Vec::new())
    }

    fn swap(&mut self, i: isize, j: isize) {
        if (i >= 0 && i < self.0.len() as isize) && (j >= 0 && j < self.0.len() as isize) {
            self.0.swap(i as usize, j as usize);
        }
    }

    fn sort_by_value(&mut self) {
        self.0.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }
}

fn main() {
    let mut om = OrderedMap::new();
    // Add elements to om here
    om.sort_by_value();
}

impl fmt::Display for OrderedMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, pair) in self.0.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{{{}, {}}}", pair.0, pair.1)?;
        }
        write!(f, "]")
    }
}
