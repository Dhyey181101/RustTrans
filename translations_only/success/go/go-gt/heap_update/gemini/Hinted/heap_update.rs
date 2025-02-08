
use std::cmp::Ordering;

#[derive(Debug)]
struct Heap {
    n: i64,
    i: Vec<i64>,
    a: Vec<i64>,
    w: Vec<i64>,
}

#[derive(Debug)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

impl Heap {
    fn update(&mut self, p: &mut [i64], i: i64, g: &Matrix) {
        let mut j = 0;
        while j < g.n {
            if g.get(i, j) > 0 {
                if self.w[i as usize] + g.get(i, j) < self.w[j as usize] {
                    p[j as usize] = i + 1;
                    self.w[j as usize] = self.w[i as usize] + g.get(i, j);
                    self.up(j);
                }
            }
            j += 1;
        }
    }

    fn up(&mut self, mut j: i64) {
        loop {
            let i = (j - 1) / 2;
            match self.less(i, j) {
                Ordering::Equal | Ordering::Greater => break,
                Ordering::Less => {
                    self.swap(i, j);
                    j = i;
                }
            }
        }
    }

    fn swap(&mut self, a: i64, b: i64) {
        let i = self.i[a as usize];
        let j = self.i[b as usize];
        self.i[a as usize] = self.i[b as usize];
        self.i[b as usize] = i;
        self.a[i as usize] = b;
        self.a[j as usize] = a;
    }

    fn less(&self, a: i64, b: i64) -> Ordering {
        let i = self.i[a as usize];
        let j = self.i[b as usize];
        self.w[i as usize].cmp(&self.w[j as usize])
    }
}

