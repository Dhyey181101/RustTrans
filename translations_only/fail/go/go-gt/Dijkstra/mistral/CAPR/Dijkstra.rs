
use std::collections::BinaryHeap;
use std::usize;

type Matrix = Vec<Vec<i64>>;
type Heap = BinaryHeap<Node>;

const INF: i64 = i64::MAX;

#[derive(Eq, PartialEq)]
struct Node {
    dist: i64,
    id: usize,
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
    h.push(Node { dist: 0, id: i });
    while let Some(mut n) = h.pop() {
        if p[n.id] == INF {
            p[n.id] = n.dist;
            if h.is_empty() {
                break;
            }
            n = h.pop().unwrap();
        }
        update(&mut h, &mut p, G, n.id);
    }
    p
}

fn new_heap(n: usize) -> Heap {
    let mut h = Heap::new();
    for i in 0..n {
        h.push(Node { dist: INF, id: i });
    }
    h
}

fn update(h: &mut Heap, p: &mut Vec<i64>, G: &Matrix, i: usize) {
    for j in 0..G.len() {
        if G[i][j] > 0 {
            let new_dist = p[i] + G[i][j];
            if new_dist < p[j] {
                p[j] = new_dist;
                let n = Node { dist: new_dist, id: j };
                h.push(n);
            }
        }
    }
}

fn get(m: &Matrix, i: usize, j: usize) -> i64 {
    m[i][j]
}
