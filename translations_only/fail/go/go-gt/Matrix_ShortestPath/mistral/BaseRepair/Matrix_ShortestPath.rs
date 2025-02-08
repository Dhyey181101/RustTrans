
use std::collections::LinkedList;

type Matrix = Vec<Vec<i64>>;

fn shortest_path(g: &Matrix, src: i64, tar: i64, n: &Matrix) -> LinkedList<i64> {
    let mut p = LinkedList::new();
    if g[src as usize][tar as usize] == 0 {
        return p;
    }
    let next = n[src as usize][tar as usize];
    if next == 0 {
        p.push_back(tar);
    } else {
        let mut back = shortest_path(g, src, next - 1, n);
        p.append(&mut back);
        let mut front = shortest_path(g, next - 1, tar, n);
        p.append(&mut front);
    }
    p
}

fn get(m: &Matrix, i: i64, j: i64) -> i64 {
    m[i as usize][j as usize]
}

fn main() {
    // Example usage
}
