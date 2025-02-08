

use std::mem::swap;

struct Heap {
    i: Vec<usize>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn up(&mut self, j: usize) {
        let mut j = j;
        while j > 0 {
            let i = (j - 1) / 2;
            if i == j || self.less(i, j) {
                break;
            }
            self.swap(i, j);
            j = i;
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let i = self.i[a];
        self.i[a] = self.i[b];
        self.i[b] = i;
        let temp = self.a[i];
        self.a[i] = self.a[self.i[b]];
        self.a[self.i[b]] = temp;
    }

    fn less(&self, a: usize, b: usize) -> bool {
        self.w[self.i[a]] < self.w[self.i[b]]
    }
}

fn main() {
    let mut h = Heap {
        i: vec![0],
        a: vec![670014898176],
        w: vec![171523813933056],
    };
    h.up(0);
}

