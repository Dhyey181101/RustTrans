
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

fn dijkstra(g: &Matrix, i: i64) -> Vec<i64> {
    let mut p = vec![0; g.n as usize];
    let mut h = BinaryHeap::new();
    h.push(HeapNode {
        dist: 0,
        node: i,
    });
    let mut dist = HashMap::new();
    dist.insert(i, 0);
    while !h.is_empty() {
        let HeapNode { dist: d, node: i } = h.pop().unwrap();
        if d == i64::MAX {
            return p;
        }
        for (j, w) in g.get(i).iter().enumerate() {
            if *w > 0 {
                let new_dist = d + w;
                if !dist.contains_key(&(j as i64)) || new_dist < *dist.get(&(j as i64)).unwrap() {
                    dist.insert(j as i64, new_dist);
                    h.push(HeapNode {
                        dist: new_dist,
                        node: j as i64,
                    });
                    p[j] = i + 1;
                }
            }
        }
    }
    p
}

struct HeapNode {
    dist: i64,
    node: i64,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for HeapNode {}

struct Matrix {
    n: i64,
    a: Vec<i64>,
}

impl Matrix {
    fn get(&self, i: i64) -> Vec<i64> {
        let start = (i * self.n) as usize;
        let end = start + self.n as usize;
        self.a[start..end].to_vec()
    }
}
