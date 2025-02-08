
struct Heap {
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn up(&mut self, mut j: i64) {
        loop {
            let i = (j - 1) / 2;
            if i == j || self.less(i, j) {
                break;
            }
            self.swap(i, j);
            j = i;
        }
    }

    fn swap(&mut self, a: i64, b: i64) {
        let (i, j) = (self.i[a as usize], self.i[b as usize]);
        self.i.swap(a as usize, b as usize);
        self.a[i as usize] = b;
        self.a[j as usize] = a;
    }

    fn less(&self, a: i64, b: i64) -> bool {
        let (i, j) = (self.i[a as usize], self.i[b as usize]);
        self.w[i as usize] < self.w[j as usize]
    }
}
