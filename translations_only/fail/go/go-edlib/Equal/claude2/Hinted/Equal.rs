
use std::convert::TryInto;

fn equal(a: &[u32], b: &[u32]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).all(|(x, y)| x == y)
}

