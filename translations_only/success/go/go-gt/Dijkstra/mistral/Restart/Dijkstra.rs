
use std::collections::BinaryHeap;
use std::usize;

type Matrix = Vec<Vec<i64>>;
type Heap = BinaryHeap<Node>;

const INF: i64 = i64::MAX;

#[derive(Eq, PartialEq)]
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
    let n = G.len();
    let mut p = vec![INF; n];
    let mut h = Heap::new();
    h.push(Node { dist: 0, index: i });
    while let Some(mut node) = h.pop() {
        if p[node.index] != INF {
            continue;
        }
        p[node.index] = node.dist;
        for j in 0..n {
            if G[node.index][j] > 0 {
                h.push(Node {
                    dist: node.dist + G[node.index][j],
                    index: j,
                });
            }
        }
    }
    p
}
