
use std::collections::LinkedList;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

fn shortest_path(matrix: &Matrix, src: i64, tar: i64, n: &Matrix) -> LinkedList<i64> {
    let mut p = LinkedList::new();
    if matrix.get(src, tar) == 0 {
        return p;
    }
    let next = n.get(src, tar);
    if next == 0 {
        p.push_back(tar);
    } else {
        let mut first_half = shortest_path(matrix, src, next - 1, n);
        let mut second_half = shortest_path(matrix, next - 1, tar, n);
        p.append(&mut first_half);
        p.append(&mut second_half);
    }
    p
}
