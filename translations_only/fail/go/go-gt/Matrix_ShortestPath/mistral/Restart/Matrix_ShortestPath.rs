
use std::collections::LinkedList;

type Int64 = i64;

struct Matrix {
    n: Int64,
    a: Vec<Int64>,
}

impl Matrix {
    fn get(&self, i: Int64, j: Int64) -> Int64 {
        self.a[i as usize * self.n as usize + j as usize]
    }
}

fn shortest_path(g: &Matrix, src: Int64, tar: Int64, n: &Matrix) -> LinkedList<Int64> {
    let mut p = LinkedList::new();
    if g.get(src, tar) == 0 {
        return p;
    }
    let next = n.get(src, tar);
    if next == 0 {
        p.push_back(tar);
    } else {
        let mut next_list = shortest_path(g, src, next - 1, n);
        p.append(&mut next_list);
        let mut tar_list = shortest_path(g, next - 1, tar, n);
        p.append(&mut tar_list);
    }
    p
}

// The rest of the code is omitted since it's not directly related to the function.
// You can implement the rest of the structs and methods if needed.
