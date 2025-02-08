

use std::boxed::Box;

type Pair = (String, f32);

fn swap(a: &mut Pair, b: &mut Pair) {
    let temp = (*a).1;
    (*a).1 = (*b).1;
    (*b).1 = temp;
}

struct OrderedMap(Box<[Pair]>);

impl OrderedMap {
    fn new() -> Self {
        OrderedMap(Box::new([("".to_string(), 0.0); 0]))
    }

    fn swap_elements(&mut self, i: usize, j: usize) {
        let temp = self.0[i].clone();
        self.0[i] = self.0[j].clone();
        self.0[j] = temp;
    }
}

fn main() {
    let mut map = OrderedMap::new();
    map.swap_elements(0, 1);
}

