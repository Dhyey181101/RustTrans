
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        weight: 0,
        index: i as usize,
    });

    while let Some(Node { weight, index }) = heap.pop() {
        let index = index as i64;
        if weight == std::i64::MAX {
            break;
        }
        p[index as usize] = index + 1;
        for (j, &weight_j) in g.a.iter().enumerate().skip(index as usize * g.n as usize) {
            if weight_j > 0 {
                let j = j as i64 - index * g.n;
                if weight + weight_j < p[j as usize] {
                    p[j as usize] = weight + weight_j;
                    heap.push(Node {
                        weight: p[j as usize],
                        index: j as usize,
                    });
                }
            }
        }
    }

    p
}

#[derive(PartialEq, Eq)]
struct Node {
    weight: i64,
    index: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

fn main() {}
