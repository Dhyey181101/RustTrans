
use std::collections::LinkedList;

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[(i * m.n + j) as usize]
}

fn shortest_path(g: &Matrix, src: i64, tar: i64, n: &Matrix) -> LinkedList<i64> {
    let mut p = LinkedList::new();
    if get(g, src, tar) == 0 {
        return p;
    }
    let next = get(n, src, tar);
    if next == 0 {
        p.push_back(tar);
    } else {
        p.append(&mut shortest_path(g, src, next - 1, n));
        p.append(&mut shortest_path(g, next - 1, tar, n));
    }
    p
}
