
use std::cmp::Ordering;

fn min(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        Ordering::Less => a,
        _ => b,
    }
}
