
use rand::Rng;
use std::fmt;

const NOT_VISITED: i64 = -1;

type Vector = Vec<i64>;
type Matrix = Vec<Vec<i64>>;

fn local_search(a: &Matrix, b: &Matrix, p: &mut Vector, cost: &mut i64) {
    let n = p.len() as i64;
    // ... rest of the function implementation
}
