

use std::collections::BinaryHeap;
use std::usize;
use std::boxed::Box;

type Matrix = Vec<Vec<i64>>;
type Heap = BinaryHeap<Node>;

const INF: i64 = i64::MAX;

#[derive(Eq, PartialEq, Clone)]
struct Node {
    dist: i64,
    index: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn Dijkstra(G: &Matrix, i: usize) -> Vec<i64> {
    let mut p = vec![INF; G.len()];
    let mut h = Heap::new();
    h.push(Node { dist: 0, index: i });
    while let Some(mut node) = h.pop() {
        if p[node.index] == node.dist {
            continue;
        }
        p[node.index] = node.dist;
        for j in 0..G.len() {
            if G[node.index][j] > 0 {
                let dist = node.dist + G[node.index][j];
                if dist < p[j] {
                    h.push(Node { dist, index: j });
                }
            }
        }
    }
    p
}

fn new_heap(n: usize) -> Heap {
    let mut h = Heap::new();
    for i in 0..n {
        h.push(Node { dist: INF, index: i });
    }
    h
}

fn update_heap(h: &mut Heap, p: &mut Vec<i64>, i: usize, G: &Matrix) {
    let mut new_nodes = Vec::new();
    for j in 0..G.len() {
        if G[i][j] > 0 {
            let dist = h.peek().map(|n| n.dist).unwrap_or(INF) + G[i][j];
            if dist < p[j] {
                new_nodes.push(Node { dist, index: j });
            }
        }
    }
    *h = new_nodes.into_iter().chain(h.clone().into_iter()).collect();
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m[i][j]
}

