
use std::cmp::Ordering;

pub struct Heap {
    pub n: i64,
    pub i: Vec<i64>,
    pub a: Vec<usize>,
    pub w: Vec<i64>,
}

impl Heap {
    pub fn pop(&mut self) -> i64 {
        let i = self.i[0];
        self.n -= 1;
        self.swap(0, self.n as usize);
        self.down(0);
        i
    }

    pub fn swap(&mut self, a: usize, b: usize) {
        let i = self.i[a];
        let j = self.i[b];
        self.i[a] = self.i[b];
        self.i[b] = i;
        self.a[i as usize] = b;
        self.a[j as usize] = a;
    }

    pub fn down(&mut self, mut i: usize) {
        loop {
            let left = 2 * i + 1;
            if left >= self.n as usize {
                break;
            }
            let mut j = left;
            if let Some(right) = left.checked_add(1) {
                if right < self.n as usize && !self.less(left, right) {
                    j = right;
                }
            }
            if self.less(i, j) {
                break;
            }
            self.swap(i, j);
            i = j;
        }
    }

    pub fn less(&self, a: usize, b: usize) -> bool {
        let i = self.i[a];
        let j = self.i[b];
        self.w[i as usize] < self.w[j as usize]
    }
}
