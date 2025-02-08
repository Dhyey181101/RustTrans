

use std::ops::Index;

struct Heap {
    n: i64,
    i: Vec<usize>,
    a: Vec<i64>,
    w: Vec<i64>,
}

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Index<usize> for Matrix {
    type Output = i64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.a[index]
    }
}

fn up(heap: &mut Heap, j: usize) {
    let mut i_j = heap.i[j];
    let mut i_i = heap.i[j];
    let mut i_parent = (j - 1) / 2;
    while i_parent != j {
        let i_parent_elem = heap.i[i_parent];
        if heap.less(i_parent_elem, i_i) {
            heap.i[j] = i_parent_elem;
            heap.i[i_parent] = i_j;
            i_i = i_parent_elem;
            i_j = heap.i[j];
            i_parent = (j - 1) / 2;
        } else {
            break;
        }
    }
    heap.i[j] = i_i;
}

impl Heap {
    fn update(&mut self, p: &mut [i64], i: i64, g: &Matrix) {
        let mut j = 0;
        for _ in 0..g.n {
            if g[j * g.n as usize + i as usize] > 0 {
                if self.w[i as usize] + g[j * g.n as usize + i as usize] < self.w[j as usize] {
                    p[j as usize] = i + 1;
                    self.w[j as usize] = self.w[i as usize] + g[j * g.n as usize + i as usize];
                    up(self, j);
                }
            }
            j += 1;
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let temp = self.a[self.i[a]];
        self.a[self.i[a]] = self.a[self.i[b]];
        self.a[self.i[b]] = temp;
        let temp_w = self.w[a];
        self.w[a] = self.w[b];
        self.w[b] = temp_w;
    }

    fn less(&self, a: usize, b: usize) -> bool {
        self.w[a] < self.w[b]
    }
}

fn main() {}

