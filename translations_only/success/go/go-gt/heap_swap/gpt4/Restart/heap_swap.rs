
struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
}

impl Heap {
    fn swap(&mut self, a: i64, b: i64) {
        let (i, j) = (self.i[a as usize], self.i[b as usize]);
        self.i[a as usize] = self.i[b as usize];
        self.i[b as usize] = self.i[a as usize];
        self.a[i as usize] = b;
        self.a[j as usize] = a;
    }
}

fn main() {}
