
use std::collections::LinkedList;

fn shortest_path(src: i64, tar: i64, g: &Matrix, n: &Matrix) -> Option<LinkedList<i64>> {
    let mut p = LinkedList::new();
    if g.a[src as usize * g.n as usize + tar as usize] == 0 {
        return None;
    }
    let next = n.a[src as usize * n.n as usize + tar as usize];
    if next == 0 {
        p.push_back(tar);
    } else {
        p.append(&mut shortest_path(src, next - 1, g, n).unwrap());
        p.append(&mut shortest_path(next - 1, tar, g, n).unwrap());
    }
    Some(p)
}

fn main() {
    let g = Matrix { n: 386, a: vec![0] };
    let n = Matrix { n: 0, a: vec![4608] };
    let result = shortest_path(720586978448443914, 9169622380866832896, &g, &n);
    match result {
        Some(_) => println!("Valid input"),
        None => println!("Invalid input"),
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}
