
use std::collections::LinkedList;

pub fn shortest_path(g: &Matrix, src: i64, tar: i64, n: &Matrix) -> Option<LinkedList<i64>> {
    let mut p = LinkedList::new();
    if g.a[(src * g.n + tar) as usize] == 0 {
        return None;
    }
    let next = n.a[(src * n.n + tar) as usize];
    if next == 0 {
        p.push_back(tar);
    } else {
        p.append(&mut shortest_path(g, src, next - 1, n).unwrap());
        p.append(&mut shortest_path(g, next - 1, tar, n).unwrap());
    }
    Some(p)
}

pub fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

pub struct Matrix {
    pub n: i64,
    pub a: Vec<i64>,
}
