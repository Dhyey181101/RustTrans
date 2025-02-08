
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

impl Heap {
    fn new(n: i64, i: Vec<i64>, a: Vec<i64>, w: Vec<i64>) -> Self {
        Heap { n, i, a, w }
    }

    fn less(&self, a: usize, b: usize) -> bool {
        let i = self.i[a] as usize;
        let j = self.i[b] as usize;
        self.w[i] < self.w[j]
    }

    fn swap(&mut self, a: usize, b: usize) {
        let i = self.i[a] as usize;
        let j = self.i[b] as usize;
        self.i.swap(a, b);
        self.a.swap(i, j);
    }
}

fn down(h: &mut Heap, mut i: i64) {
    loop {
        let left = 2 * i + 1;
        if left >= h.n {
            break;
        }
        let mut j = left;
        let right = left + 1;
        if right < h.n && !h.less(left as usize, right as usize) {
            j = right;
        }
        if h.less(i as usize, j as usize) {
            break;
        }
        h.swap(i as usize, j as usize);
        i = j;
    }
}

fn main() {
    let mut heap = Heap::new(10, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    down(&mut heap, 0);
}
