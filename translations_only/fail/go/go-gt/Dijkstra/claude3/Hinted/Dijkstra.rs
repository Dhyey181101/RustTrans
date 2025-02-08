

use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut heap = BinaryHeap::new();
    heap.push(Node { i, w: 0 });

    while let Some(Node { i, w }) = heap.pop() {
        if w == std::i64::MAX {
            return p;
        }
        update(&mut p, i, w, &g, &mut heap);
    }

    p
}

fn update(p: &mut Vec<i64>, i: i64, w: i64, g: &Matrix, heap: &mut BinaryHeap<Node>) {
    for j in 0..g.n {
        if g.get(i, j) > 0 {
            let new_w = w + g.get(i, j);
            if let Some(node) = heap.iter().find(|n| n.i == j) {
                if new_w < node.w {
                    p[(j as usize)] = i + 1;
                    heap.push(Node { i: j, w: new_w });
                }
            } else {
                p[(j as usize)] = i + 1;
                heap.push(Node { i: j, w: new_w });
            }
        }
    }
}

#[derive(PartialEq, Eq)]
struct Node {
    i: i64,
    w: i64,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.w.cmp(&self.w)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Matrix {
    n: i64,
    a: Box<[i64]>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

