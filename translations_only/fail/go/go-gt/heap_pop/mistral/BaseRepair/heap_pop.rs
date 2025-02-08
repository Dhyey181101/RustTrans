

use std::mem;

struct Heap {
    n: i64,
    i: Vec<usize>,
    a: Vec<Box<i64>>,
    w: Vec<i64>,
}

impl Heap {
    fn pop(&mut self) -> i64 {
        let i = self.i[0];
        self.n -= 1;
        self.swap(0, self.n as usize);
        self.down(&mut 0);
        *self.a[i as usize]
    }

    fn swap(&mut self, a: usize, b: usize) {
        let temp = self.i[a];
        self.i[a] = self.i[b];
        self.i[b] = temp;
    }

    fn down(&mut self, i: &mut usize) {
        loop {
            let left = 2 * *i + 1;
            if left >= self.n as usize {
                break;
            }
            let mut j = left;
            let right = left + 1;
            if right < self.n as usize && !self.less(left, right) {
                j = right;
            }
            if self.less(*i, j) {
                break;
            }
            self.swap(*i, j);
            *i = j;
        }
    }

    fn less(&self, a: usize, b: usize) -> bool {
        let i = self.i[a];
        let j = self.i[b];
        self.w[i as usize] < self.w[j as usize]
    }
}

fn main() {
    let mut h = Heap {
        n: 5,
        i: vec![0, 1, 2, 3, 4],
        a: vec![Box::new(1), Box::new(2), Box::new(3), Box::new(4), Box::new(5)],
        w: vec![1, 2, 3, 4, 5],
    };
    println!("{}", h.pop());
}

