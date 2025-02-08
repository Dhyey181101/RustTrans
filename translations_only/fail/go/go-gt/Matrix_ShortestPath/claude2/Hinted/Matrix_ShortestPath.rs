
use std::collections::LinkedList;

struct Matrix {
    n: i64,
    a: Vec<i64>, 
}

fn shortest_path(src: i64, tar: i64, n: &Matrix, g: &Matrix) -> LinkedList<i64> {
    let mut p = LinkedList::new();
    
    if get(g, src, tar) == 0 {
        return p;
    }
    
    let next = get(n, src, tar);
    if next == 0 {
        p.push_back(tar);
    } else {
        p.append(&mut shortest_path(src, next - 1, n, g)); 
        p.append(&mut shortest_path(next - 1, tar, n, g));
    }
    
    p
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m.a[i as usize * m.n as usize + j as usize]
}

