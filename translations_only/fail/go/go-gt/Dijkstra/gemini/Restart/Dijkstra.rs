
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut h = BinaryHeap::with_capacity(g.n as usize);
    h.push(HeapNode {
        i,
        w: 0,
    });
    while !h.is_empty() {
        let HeapNode { i, w } = h.pop().unwrap();
        if w == i64::MAX {
            return p;
        }
        for j in 0..g.n {
            if g.get(i, j) > 0 {
                let new_w = w + g.get(i, j);
                if new_w < h.peek().unwrap().w {
                    p[j as usize] = i + 1;
                    h.push(HeapNode { i: j, w: new_w });
                }
            }
        }
    }
    p
}

struct HeapNode {
    i: i64,
    w: i64,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.w.cmp(&other.w).reverse()
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl Eq for HeapNode {}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}
