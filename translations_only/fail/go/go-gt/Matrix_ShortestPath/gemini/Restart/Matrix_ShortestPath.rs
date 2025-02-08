
use std::collections::LinkedList;

pub fn shortest_path(src: i64, tar: i64, n: &Matrix) -> LinkedList<i64> {
    let mut p = LinkedList::new();
    if n.a[src as usize * n.n as usize + tar as usize] == 0 {
        return p;
    }
    let next = n.a[src as usize * n.n as usize + tar as usize];
    if next == 0 {
        p.push_back(tar);
    } else {
        p.append(&mut shortest_path(src, next - 1, n));
        p.append(&mut shortest_path(next - 1, tar, n));
    }
    p
}

pub fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[i as usize * m.n as usize + j as usize]
}

pub struct Matrix {
    n: i64,
    a: Vec<i64>,
}
