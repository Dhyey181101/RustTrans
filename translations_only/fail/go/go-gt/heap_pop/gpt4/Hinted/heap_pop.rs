
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn pop(&mut self) -> i64 {
        let i = self.i[0];
        self.n -= 1;
        self.swap(0, self.n);
        self.down(0);
        i
    }

    fn swap(&mut self, a: i64, b: i64) {
        let (i, j) = (self.i[a as usize], self.i[b as usize]);
        self.i.swap(a as usize, b as usize);
        self.a[i as usize] = b;
        self.a[j as usize] = a;
    }

    fn down(&mut self, mut i: i64) {
        loop {
            let left = 2 * i + 1;
            if left >= self.n {
                break;
            }
            let mut j = left;
            let right = left + 1;
            if right < self.n && !self.less(left, right) {
                j = right;
            }
            if self.less(i, j) {
                break;
            }
            self.swap(i, j);
            i = j;
        }
    }

    fn less(&self, a: i64, b: i64) -> bool {
        let (i, j) = (self.i[a as usize], self.i[b as usize]);
        self.w[i as usize] < self.w[j as usize]
    }
}
