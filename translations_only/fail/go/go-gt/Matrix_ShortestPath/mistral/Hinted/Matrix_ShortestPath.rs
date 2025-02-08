

use std::collections::LinkedList;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

struct ContainerList {
    root: LinkedList<i64>,
}

struct ContainerListElement {
    value: i64,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[i as usize * self.n as usize + j as usize]
    }

    fn shortest_path(&self, src: i64, tar: i64, N: &Matrix) -> LinkedList<i64> {
        let mut p = LinkedList::new();
        if self.get(src, tar) == 0 {
            return p;
        }
        let next = N.get(src, tar);
        if next == 0 {
            p.push_back(tar);
        } else {
            let mut next_list = self.shortest_path(src, next - 1, N);
            p.append(&mut next_list);
            p.append(&mut self.shortest_path(next - 1, tar, N));
        }
        p
    }
}

fn main() {
    // Add your main function implementation here
}

