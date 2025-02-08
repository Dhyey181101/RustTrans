

use std::mem::swap;

struct Heap {
    i: Vec<usize>,
    a: Vec<Box<i64>>,
    w: Vec<i64>,
}

impl Heap {
    fn up(&mut self, j: usize) {
        let mut j = j;
        loop {
            let i = (j - 1) / 2;
            if i == j || self.less(i, j) {
                break;
            }
            self.swap(i, j);
            j = i;
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let i = self.i[a].clone();
        self.i[a] = self.i[b].clone();
        self.i[b] = i;
        let temp_a = self.a[a].clone();
        self.a[a] = self.a[b].clone();
        self.a[b] = temp_a;
    }

    fn less(&self, a: usize, b: usize) -> bool {
        self.w[self.i[a]] < self.w[self.i[b]]
    }
}

fn main() {}

