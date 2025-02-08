
use std::collections::HashSet;

pub fn union(a: &[String], b: &[String]) -> Vec<Vec<char>> {
    let mut set = HashSet::new();
    for item in a {
        set.insert(item.clone());
    }
    for item in b {
        if !set.contains(item) {
            set.insert(item.clone());
        }
    }

    let mut out = Vec::new();
    for word in set {
        out.push(word.chars().collect());
    }
    out
}
