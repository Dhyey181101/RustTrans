
use std::collections::LinkedList;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }

    fn shortest_path(&self, src: i64, tar: i64, n: &Matrix) -> LinkedList<i64> {
        let mut p = LinkedList::new();
        if self.get(src, tar) == 0 {
            return p;
        }
        let next = n.get(src, tar);
        if next == 0 {
            p.push_back(tar);
        } else {
            p.append(&mut self.shortest_path(src, next - 1, n));
            p.append(&mut self.shortest_path(next - 1, tar, n));
        }
        p
    }
}
