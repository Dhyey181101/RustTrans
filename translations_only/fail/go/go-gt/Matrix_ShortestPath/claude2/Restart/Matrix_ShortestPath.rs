
use std::collections::LinkedList;

struct Matrix {
    n: usize,
    a: Vec<i64>,
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m.a[i*m.n + j]
}

fn shortest_path(g: &Matrix, src: usize, tar: usize, n: &Matrix) -> LinkedList<usize> {
    let mut p = LinkedList::new();
    if get(g, src, tar) == 0 {
        return p;
    }
    
    let next = get(n, src, tar) as usize;
    if next == 0 {
        p.push_back(tar);
    } else {
        p.append(&mut shortest_path(g, src, next-1, n));
        p.append(&mut shortest_path(g, next-1, tar, n));
    }
    
    p
}

