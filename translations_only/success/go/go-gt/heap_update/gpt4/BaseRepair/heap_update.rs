
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Heap {
    fn update(&mut self, p: &mut Vec<i64>, i: i64, g: &Matrix) {
        for j in 0..g.n {
            if g.get(i, j) > 0 {
                if self.w[i as usize] + g.get(i, j) < self.w[j as usize] {
                    p[j as usize] = i + 1;
                    self.w[j as usize] = self.w[i as usize] + g.get(i, j);
                    self.up(self.a[j as usize]);
                }
            }
        }
    }

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
        self.i[a as usize] = self.i[b as usize];
        self.i[b as usize] = self.i[a as usize];
        self.a[i as usize] = b;
        self.a[j as usize] = a;
    }

    fn less(&self, a: i64, b: i64) -> bool {
        let (i, j) = (self.i[a as usize], self.i[b as usize]);
        self.w[i as usize] < self.w[j as usize]
    }
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn main() {}
