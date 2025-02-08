
use std::cmp::Ordering;

pub struct Heap {
    pub n: i64,
    pub i: Vec<i64>,
    pub a: Vec<i64>,
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
        self.a[i as usize] = a as i64;
        self.a[j as usize] = b as i64;
    }

    pub fn down(&mut self, mut i: usize) {
        loop {
            let left = 2 * i + 1;
            if left >= self.n as usize {
                break;
            }
            let mut j = left;
            let right = left + 1;
            if right < self.n as usize && self.less(right, left) == Ordering::Less {
                j = right;
            }
            if self.less(i, j) == Ordering::Greater {
                break;
            }
            self.swap(i, j);
            i = j;
        }
    }

    pub fn less(&self, a: usize, b: usize) -> Ordering {
        self.w[self.i[a] as usize].cmp(&self.w[self.i[b] as usize])
    }
}

