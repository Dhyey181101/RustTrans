
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut heap = BinaryHeap::new();
    heap.push((0, i));

    while let Some((weight, node)) = heap.pop() {
        if weight == std::i64::MAX {
            return p;
        }

        for (j, &edge_weight) in g.a.iter().enumerate().skip(node as usize * g.n as usize) {
            if edge_weight > 0 {
                let j = j as i64 % g.n;
                let new_weight = weight + edge_weight;
                if new_weight < p[j as usize] {
                    p[j as usize] = new_weight;
                    heap.push((new_weight, j));
                }
            }
        }
    }

    p
}

#[derive(Clone)]
struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64, j: i64) -> i64 {
        self.a[(i * self.n + j) as usize]
    }
}

#[derive(Eq, PartialEq)]
struct Node(i64, i64);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
